use std::collections::HashMap;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::{BufWriter, Read, SeekFrom, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use bitbit::{BitReader, MSB};
use clap::{Parser, Subcommand};
use huff::bitwriter::BitWriter;
use huff::encode;
use huff::tree::Tree;

#[derive(Subcommand)]
enum Command {
    /// Compress the input file
    Compress {
        #[clap(short, long, value_parser, value_name = "INPUT-FILE")]
        input_file_name: PathBuf,
        #[clap(short, long, value_parser, value_name = "OUTPUT-FILE")]
        output_file_name: PathBuf,
        #[clap(short, long, value_parser, value_name = "verbose")]
        verbose: bool,
    },
    /// Decompress the input file
    Decompress {
        #[clap(short, long, value_parser, value_name = "INPUT-FILE")]
        input_file_name: PathBuf,

        #[clap(short, long, value_parser, value_name = "OUTPUT-FILE")]
        output_file_name: PathBuf,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Basic huffman (de-)compressor
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Compress {
            input_file_name,
            output_file_name,
            verbose,
        } => {
            subcommand_compress(&input_file_name, output_file_name, verbose)
                .with_context(|| format!("Error compressing file {:?}", input_file_name))?;
        }
        Command::Decompress {
            input_file_name,
            output_file_name,
        } => {
            subcommand_decompress(&input_file_name, output_file_name)
                .with_context(|| format!("Error decompressing file {:?}", input_file_name))?;
        }
    }

    Ok(())
}

fn subcommand_compress(input_file: &PathBuf, output_file: PathBuf, verbose: bool) -> Result<()> {
    let mut f = File::open(input_file)?;
    let metadata = metadata(input_file)?;
    if !metadata.is_file() {
        return Err(anyhow!("{:?} is a not valid file.", input_file));
    }

    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer)?;

    let (table, tree) = encode(&buffer);

    if verbose {
        print_mapping_table(&table);
    }

    let w = BufWriter::new(File::create(&output_file)?);
    let mut bw = BitWriter::new(w);

    write_tree(&mut bw, &tree)?;
    let padding_tree = bw.align()?;

    compress(&mut bw, &table, &buffer)?;
    let padding_data = bw.align()?;

    let last_byte = (padding_tree << 4) | padding_data;
    bw.write_byte(last_byte)?;

    Ok(())
}

fn subcommand_decompress(input_file: &PathBuf, output_file: PathBuf) -> Result<()> {
    let mut r = File::open(input_file)?;
    r.seek(SeekFrom::End(-1))?;
    let mut buf = vec![];
    r.read_to_end(&mut buf)?;

    let padding_tree = buf[0] >> 4 & 0xF;
    let padding_data = buf[0] & 0xF;

    let r = File::open(input_file)?;
    let mut br: BitReader<_, MSB> = BitReader::new(r);
    let tree = read_tree(&mut br).unwrap();

    br.read_bits(padding_tree as usize)?;

    let current_pos = br.get_ref().seek(SeekFrom::Current(1))?;
    let end_pos = br.get_ref().seek(SeekFrom::End(0))?;

    let bits_to_read = (end_pos - current_pos) * 8 - padding_data as u64;

    br.get_ref().seek(SeekFrom::Start(current_pos - 1))?;

    let decompressed = extract(&mut br, &tree, bits_to_read)?;

    let mut w = File::create(output_file)?;

    w.write_all(&decompressed)?;

    Ok(())
}

fn read_leaf<T: Read>(br: &mut BitReader<T, MSB>) -> Result<Tree> {
    Ok(Tree::Leaf {
        value: 0,
        byte: br.read_byte()?,
    })
}

fn read_tree<T: Read>(br: &mut BitReader<T, MSB>) -> Option<Tree> {
    let bit = br.read_bit().unwrap();
    if bit {
        let l = read_tree(br)?;
        let r = read_tree(br)?;

        Some(Tree::Branch {
            value: 0,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        })
    } else {
        Some(read_leaf(br).unwrap())
    }
}

fn write_tree<T: Write>(bw: &mut BitWriter<T>, tree: &Tree) -> Result<()> {
    match tree {
        Tree::Leaf { byte, .. } => {
            bw.write_bit(false)?;
            bw.write_byte(*byte)?;
        }
        Tree::Branch { left, right, .. } => {
            bw.write_bit(true)?;
            if left.is_some() {
                write_tree(bw, left.as_ref().unwrap())?;
            }
            if right.is_some() {
                write_tree(bw, right.as_ref().unwrap())?;
            }
        }
    }

    Ok(())
}

fn extract<T: Read>(br: &mut BitReader<T, MSB>, tree: &Tree, bits_to_read: u64) -> Result<Vec<u8>> {
    let mut pointer = tree;
    let mut vec = vec![];

    for _ in 0..bits_to_read {
        let bit = br.read_bit()?;
        match bit {
            false => {
                if let Tree::Branch {
                    left: Some(left), ..
                } = pointer
                {
                    pointer = left.as_ref();
                }
            }
            true => {
                if let Tree::Branch {
                    right: Some(right), ..
                } = pointer
                {
                    pointer = right.as_ref();
                }
            }
        }

        if let Tree::Leaf { byte, .. } = pointer {
            vec.push(*byte);
            pointer = tree;
        }
    }

    Ok(vec)
}

fn compress<T: Write>(
    bw: &mut BitWriter<T>,
    table: &HashMap<u8, Vec<bool>>,
    data: &[u8],
) -> Result<()> {
    for byte in data {
        let bytes = table.get(byte).unwrap();
        for bit in bytes {
            bw.write_bit(*bit)?;
        }
    }

    Ok(())
}

fn print_mapping_table(table: &HashMap<u8, Vec<bool>>) {
    println!("------------------------------------");
    for (byte, encoded) in table {
        let status = if encoded.len() > 8 { "💩️" } else { "" };
        println!(
            "| {:>3} | {:08b} | {:>16}| {}",
            byte,
            byte,
            bool_vec_to_string(encoded),
            status
        )
    }
    println!("------------------------------------");
}

fn bool_vec_to_string(vec: &[bool]) -> String {
    fn m(val: &bool) -> char {
        if *val {
            '1'
        } else {
            '0'
        }
    }

    String::from_iter(vec.iter().map(m))
}
