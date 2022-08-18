use huff::encode;
use std::cmp;
use std::collections::HashMap;
use std::fs::{read_to_string, File};

use std::io::{BufReader, Read, Result, Write};

fn main() -> Result<()> {
    let text = read_to_string("readme.md")?;
    let (table, _tree) = encode(&text);

    print_mapping_table(&table);

    let bit_str_old = compress(&table, &text);

    println!("Length original: {}", text.len());
    println!(
        "Length compressed: {}",
        (bit_str_old.len() as f64 / 8.0).ceil() as usize + 1
    );
    println!("Length encoding table: ???");

    write_compressed("readme.md.bc", &bit_str_old)?;

    let compressed = read_compressed("readme.md.bc")?;

    let bit_str_new = to_bit_str(&compressed);

    assert_eq!(bit_str_old, bit_str_new);

    Ok(())
}

fn compress(table: &HashMap<char, String>, text: &str) -> String {
    let mut alloc = 0;
    for character in text.chars() {
        alloc += table.get(&character).expect("encoding info").len();
    }

    let mut bits = String::with_capacity(alloc);
    for character in text.chars() {
        bits.push_str(table.get(&character).unwrap());
    }

    bits
}

fn print_mapping_table(table: &HashMap<char, String>) {
    println!("--------------------------");
    for (character, encoded) in table {
        if *character == '\n' {
            println!("|{:>3}  |  {:>16}|", "\\n", encoded);
        } else if *character == '\t' {
            println!("|{:>3}  |  {:>16}|", "\\t", encoded);
        } else {
            println!("|{:>3}  |  {:>16}|", character, encoded);
        }
    }
    println!("--------------------------");
}

fn write_compressed(file: &str, bits: &str) -> Result<()> {
    let mut file = File::create(file)?;

    let mut consumed = 0;
    while consumed < bits.len() {
        let end = cmp::min(consumed + 8, bits.len());
        let byte = u8::from_str_radix(&bits[consumed..end], 2).unwrap();
        file.write_all(&byte.to_be_bytes())?;
        consumed = end;
    }

    // how many real bits are in the last byte
    let real_bits_in_last_byte = (bits.len() % 8) as u8;
    file.write_all(&real_bits_in_last_byte.to_be_bytes())?;

    println!("real bits in last byte: {0}", real_bits_in_last_byte);

    Ok(())
}

fn to_bit_str(compressed: &[u8]) -> String {
    let mut bits = String::with_capacity(compressed.len() * 8);
    for byte in &compressed[..compressed.len() - 2] {
        let byte = format!("{:08b}", byte);
        bits.push_str(&byte);
    }

    let last_byte_info = *compressed.last().expect("some content") as usize;
    let last_byte = compressed
        .get(compressed.len() - 2)
        .expect("enough content");

    let last_byte = format!("{:08b}", last_byte);
    bits.push_str(&last_byte[(8 - last_byte_info)..]);

    bits
}

fn read_compressed(file: &str) -> Result<Vec<u8>> {
    let file = File::open(file)?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
