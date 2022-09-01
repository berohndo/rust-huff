use std::io::*;

// "stolen"" from bitbit and modified

pub struct BitWriter<W: Write> {
    byte: u8,
    shift: usize,
    writer: W,
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> BitWriter<W> {
        BitWriter {
            byte: 0,
            shift: 0,
            writer,
        }
    }

    pub fn write_bit(&mut self, is_true: bool) -> Result<()> {
        self.byte <<= 1;
        if is_true {
            self.byte |= 1;
        }
        self.shift += 1;
        if self.shift == 8 {
            self.writer.write_all(&[self.byte])?;
            self.byte = 0;
            self.shift = 0;
        }
        Ok(())
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<()> {
        for n in 0..8 {
            let t = byte >> (7 - n) as u8;
            self.write_bit(t & 1 != 0)?;
        }

        Ok(())
    }

    pub fn align(&mut self) -> Result<u8> {
        let mut filled = 0;
        for _ in self.shift..8 {
            self.write_bit(false)?;
            filled += 1;
        }

        Ok(filled)
    }
}
