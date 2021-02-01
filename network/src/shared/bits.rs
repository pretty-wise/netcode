use std::{
    io::{Read, Write},
    slice,
};

#[derive(Debug)]
pub enum Error {
    OutOfMemory,
    InvalidArgument,
    ValueOutOfBounds,
}

pub struct BitWriter<'a> {
    dest: &'a mut [u8],
    scratch: u64,
    scratch_bits: i32,
    capacity: i64,
}

impl<'a> BitWriter<'a> {
    pub fn new(dest: &'a mut [u8]) -> BitWriter {
        let capacity = dest.len() as i64;
        BitWriter {
            dest,
            scratch: 0,
            scratch_bits: 0,
            capacity,
        }
    }

    pub fn write_byte(&mut self, value: u8) -> Result<(), Error> {
        self.write_bits(value as u32, 8)
    }

    pub fn write_bits(&mut self, value: u32, bits: i32) -> Result<(), Error> {
        if bits > 32 {
            return Err(Error::InvalidArgument);
        }
        if value > ((1u64 << bits) - 1) as u32 {
            return Err(Error::ValueOutOfBounds);
        }
        if self.available_bits() < bits as i64 {
            return Err(Error::OutOfMemory);
        }
        let value: u64 = value as u64 & ((1 << bits) - 1);
        self.scratch = (self.scratch << bits) | value;
        self.scratch_bits += bits;

        if self.scratch_bits > 32 {
            const BYTE_BITS: i32 = 8;
            let byte_count = self.scratch_bits / BYTE_BITS;

            for _ in 0..byte_count {
                let offset = self.scratch_bits - BYTE_BITS;
                let byte = (self.scratch >> offset) as u8;
                self.dest.write(slice::from_ref(&byte)).unwrap();

                self.scratch &= (1 << offset) - 1;
                self.scratch_bits -= BYTE_BITS;
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) {
        const BYTE_BITS: i32 = 8;
        let byte_count = self.scratch_bits / BYTE_BITS;

        for _ in 0..byte_count {
            let offset = self.scratch_bits - BYTE_BITS;
            let byte = (self.scratch >> offset) as u8;
            self.dest.write(slice::from_ref(&byte)).unwrap();

            self.scratch &= (1 << offset) - 1;
            self.scratch_bits -= BYTE_BITS;
        }

        // remaining bits
        let offset = BYTE_BITS - self.scratch_bits;
        let byte = (self.scratch << offset) as u8;
        self.dest.write(slice::from_ref(&byte)).unwrap();

        self.scratch &= (1 << offset) - 1;
        self.scratch_bits = 0;
    }

    pub fn written_bytes(&self) -> i64 {
        (self.capacity - self.dest.len() as i64) + (self.scratch_bits / 8) as i64
    }

    pub fn written_bits(&self) -> i64 {
        (self.capacity - self.dest.len() as i64) * 8 + self.scratch_bits as i64
    }

    fn available_bits(&self) -> i64 {
        (self.dest.len() as i64 * 8) - self.scratch_bits as i64
    }
}

pub struct BitReader<'a> {
    source: &'a [u8],
    scratch_bits: i32,
    scratch: u64,
}

impl<'a> BitReader<'a> {
    pub fn new(source: &[u8]) -> BitReader {
        BitReader {
            source,
            scratch_bits: 0,
            scratch: 0,
        }
    }

    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let value = self.read_bits(8)?;
        Ok(value as u8)
    }

    pub fn read_bits(&mut self, bits: i32) -> Result<u32, Error> {
        if bits > 32 {
            return Err(Error::InvalidArgument);
        }

        if self.available_bits() < bits {
            return Err(Error::OutOfMemory);
        }

        while self.scratch_bits < bits {
            let mut byte: u8 = 0;
            self.source.read(slice::from_mut(&mut byte)).unwrap();
            self.scratch = self.scratch << 8 | byte as u64;
            self.scratch_bits += 8;
        }

        let value = self.scratch >> (self.scratch_bits - bits);
        self.scratch &= (1 << (self.scratch_bits - bits)) - 1;
        self.scratch_bits -= bits;
        Ok(value as u32)
    }

    fn available_bits(&self) -> i32 {
        self.source.len() as i32 * 8 + self.scratch_bits
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_chacha::{ChaCha8Rng, ChaChaRng};

    use super::{BitReader, BitWriter};

    #[test]
    fn byte() {
        let expected = 3;
        let mut bytes: [u8; 1] = [0];

        {
            let mut writer = BitWriter::new(&mut bytes[..]);
            assert_eq!(writer.available_bits(), 8);
            assert_eq!(writer.written_bits(), 0);
            assert_eq!(writer.written_bytes(), 0);

            assert!(writer.write_byte(expected).is_ok());
            writer.available_bits();
            assert_eq!(writer.written_bits(), 8);
            assert_eq!(writer.written_bytes(), 1);
            writer.flush();
        }

        assert_eq!(bytes[0], expected);

        let mut reader = BitReader::new(&bytes[..]);

        let val = reader.read_byte();
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), expected);
    }

    #[test]
    fn multibyte() {
        let expected = 0x11aabbcc;
        let bits = 29;

        let mut bytes = [0u8; 8];
        let mut writer = BitWriter::new(&mut bytes);
        assert!(writer.write_bits(expected, bits).is_ok());
        assert!(writer.write_bits(expected, bits).is_ok());

        writer.flush();

        let mut reader = BitReader::new(&bytes);
        let res = reader.read_bits(bits);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
        let res = reader.read_bits(bits);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn bits() {
        let mut bytes = [0u8; 1];
        let max_bits = bytes.len() as i64 * 8;
        let mut writer = BitWriter::new(&mut bytes);

        let values: [(u32, i32); 4] = [(1, 1), (3, 2), (15, 4), (1, 1)];

        let mut bits_written: i64 = 0;
        for (value, bits) in &values[0..3] {
            assert!(writer.write_bits(*value, *bits).is_ok());
            assert_eq!(writer.written_bits(), bits_written + *bits as i64);
            assert_eq!(writer.written_bytes(), writer.written_bits() / 8);

            bits_written += *bits as i64;
            assert_eq!(writer.available_bits(), max_bits - bits_written);
        }

        let too_big: (u32, i32) = (3, 2);
        assert!(writer.write_bits(too_big.0, too_big.1).is_err());
        assert_eq!(writer.available_bits(), max_bits - bits_written);
        assert_eq!(writer.written_bits(), bits_written);
        assert_eq!(writer.written_bytes(), bits_written / 8);

        for (value, bits) in &values[3..] {
            assert!(writer.write_bits(*value, *bits).is_ok());
            assert_eq!(writer.written_bits(), bits_written + *bits as i64);
            assert_eq!(writer.written_bytes(), writer.written_bits() / 8);

            bits_written += *bits as i64;
            assert_eq!(writer.available_bits(), max_bits - bits_written);
        }

        assert!(writer.write_bits(1, 1).is_err());
        assert_eq!(writer.available_bits(), max_bits - 1 - 2 - 4 - 1);
        assert_eq!(writer.written_bits(), bits_written);
        assert_eq!(writer.written_bytes(), bits_written / 8);

        writer.flush();
        assert_eq!(writer.scratch_bits, 0);
        assert_eq!(writer.scratch, 0);
        assert_eq!(writer.written_bits(), bits_written);
        assert_eq!(writer.written_bytes(), bits_written / 8);

        let mut reader = BitReader::new(&bytes);

        for (value, bits) in &values {
            let result = reader.read_bits(*bits).unwrap();
            assert_eq!(result, *value);
        }
    }

    #[test]
    fn random() {
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        rand::thread_rng().fill(&mut seed);

        println!("seed: {:?}", seed);
        let mut rng = ChaChaRng::from_seed(seed);

        const NBYTES: usize = 1024;
        let nbits = NBYTES * 8;

        let mut values = Vec::<(i32, u32)>::new();

        let mut generated_bits = 0;
        loop {
            let num_bits = rng.gen_range(1..33);

            if generated_bits + num_bits > nbits {
                break;
            }

            let max_value = ((1usize << num_bits) - 1) as u32;
            let value = rng.gen_range(0..=max_value);

            values.push((num_bits as i32, value));
            generated_bits += num_bits as usize;
        }

        let leftover_bits = nbits - generated_bits;

        let mut bytes: [u8; NBYTES] = [0; NBYTES];
        let mut writer = BitWriter::new(&mut bytes);

        for (bits, value) in values.iter() {
            println!("{} = {}", bits, value);
            assert!(writer.write_bits(*value, *bits).is_ok());
        }
        assert_eq!(writer.available_bits(), leftover_bits as i64);
        writer.flush();

        let mut reader = BitReader::new(&bytes);

        for (bits, value) in values {
            let read = reader.read_bits(bits);
            assert!(read.is_ok());
            assert_eq!(read.unwrap(), value);
        }
    }
}
