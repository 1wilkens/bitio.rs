#![crate_name = "bitio"]
#![crate_type = "lib"]

#![feature(io)]

#[macro_use]
extern crate log;

use std::old_io;

/// Mainstructs are
/// * BitReader<R: old_io::Reader> for reading and
/// * BitWriter<W: old_io::Writer> for writing

/// A wrapping Reader reading bitwise from its source.
/// For now mainly a reimplementation of the C# version from
/// -- http://rosettacode.org/wiki/Bitwise_IO
pub struct BitReader<R> {
    buf:    isize,
    src:    R,
    start:  usize,
    end:    usize
}

impl<R: Reader> BitReader<R> {

    /// Creates a new `BitReader`
    pub fn new(source: R) -> BitReader<R> {
        BitReader {
            buf: 0,
            src: source,
            start: 0,
            end: 0
        }
    }

    /// Reads a single Bit from the `BitReader`
    pub fn read_bit(&mut self) -> old_io::IoResult<bool> {
        let bit = try!(self.read_bits(1));
        Ok(bit > 0)
    }

    /// Gets a given count of Bits from the `BitReader`
    pub fn read_bits(&mut self, bit_count: usize) -> old_io::IoResult<isize> {
        assert!(bit_count < 32);
        self.expand_buffer(bit_count as isize);

        // might not need all parentheses
        let result = (self.buf >> self.start) & ((1 << bit_count) - 1);
        self.start += bit_count;

        if self.end == self.start {
            // self.buf is empty => reset counter
            self.end = 0;
            self.start = self.end;
            self.buf = 0;
        }
        else if self.start >= 8 {
            // move bits to beginning of self.buf and decrease counters
            self.buf >>= self.start;
            self.end -= self.start;
            self.start = 0;
        }
        Ok(result)
    }

    /// Read a single byte from the `BitReader`
    pub fn read_byte(&mut self) -> old_io::IoResult<u8> {
        match self.read_bits(8) {
            Ok(byte) => Ok(byte as u8),
            _		 => Err(old_io::standard_error(old_io::EndOfFile))
        }
    }

    /// Gets a given count of bytes from the `BitReader`.
    /// Mainly here for convenience. Could be implemented more efficient.
    pub fn read_bytes(&mut self, buf: &mut [u8], byte_count: usize) -> old_io::IoResult<usize> {
        assert!(byte_count <= buf.len());

        let mut count = 0us;
        while count < byte_count {
            match self.read_byte() {
                Ok(byte) => buf[count] = byte,
                _        => return Err(old_io::standard_error(old_io::EndOfFile))
            }
            count += 1;
        }
        Ok(count)
    }

    fn buf_len(&self) -> isize {
        (self.end - self.start) as isize
    }

    fn expand_buffer(&mut self, b_count: isize) {
        assert!(b_count > 0 && b_count < 32);

        let mut bits_to_read: isize = b_count - self.buf_len();
        while bits_to_read > 0 {
            // self.buf is smaller than the requested bits => read bytes from source isizeo buf
            match self.src.read_byte() {
                Ok(byte) => {
                    self.buf |= (byte as isize) << self.end;
                    self.end += 8;
                    bits_to_read -= 8;
                    debug!("Expanded buffer! {} bits remaining", bits_to_read)
                }
                _        => panic!("Error! Unexpected EOF!")
            }
        }
    }
}

impl<R: old_io::Reader> old_io::Reader for BitReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> old_io::IoResult<usize> {
        let len = buf.len();
        self.read_bytes(buf, len)
    }
}

#[allow(dead_code)]
struct BitWriter<W> {
    buf: 	u64,
    targ: 	W,
    start:	u64,
    end:	u64
}
