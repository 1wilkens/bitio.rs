#![crate_name = "bitio"]
#![crate_type = "lib"]
#![comment = "Buffered bitwise IO for rust"]

#![feature(phase)]

#[phase(plugin, link)] extern crate log;

use std::io;

/// Mainstructs are
/// * BitReader<R: io::Reader> for reading and
/// * BitWriter<W: io::Writer> for writing

/// A wrapping Reader reading bitwise from its source.
/// For now mainly a reimplementation of the C# version from
/// -- http://rosettacode.org/wiki/Bitwise_IO
pub struct BitReader<R> {
    buf:    int,
    src:    R,
    start:  uint,
    end:    uint
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
    pub fn read_bit(&mut self) -> io::IoResult<bool> {
        let bit = try!(self.read_bits(1));
        Ok(bit > 0)
    }

    /// Gets a given count of Bits from the `BitReader`
    pub fn read_bits(&mut self, bit_count: uint) -> io::IoResult<int> {
        assert!(bit_count < 32)
        self.expand_buffer(bit_count as int);

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
    pub fn read_byte(&mut self) -> io::IoResult<u8> {
        match self.read_bits(8) {
            Ok(byte) => Ok(byte as u8),
            _		 => Err(io::standard_error(io::EndOfFile))
        }
    }

    /// Gets a given count of bytes from the `BitReader`.
    /// Mainly here for convenience. Could be implemented more efficient.
    pub fn read_bytes(&mut self, buf: &mut [u8], byte_count: uint) -> io::IoResult<uint> {
        assert!(byte_count <= buf.len())

        let mut count = 0u;
        while count < byte_count {
            match self.read_byte() {
                Ok(byte) => buf[count] = byte,
                _        => return Err(io::standard_error(io::EndOfFile))
            }
            count += 1;
        }
        Ok(count)
    }

    fn buf_len(&self) -> int {
        (self.end - self.start) as int
    }

    fn expand_buffer(&mut self, b_count: int) {
        assert!(b_count > 0 && b_count < 32)

        let mut bits_to_read: int = b_count - self.buf_len();
        while bits_to_read > 0 {
            // self.buf is smaller than the requested bits => read bytes from source into buf
            match self.src.read_byte() {
                Ok(byte) => {
                    self.buf |= byte as int << self.end;
                    self.end += 8;
                    bits_to_read -= 8;
                    debug!("Expanded buffer! {} bits remaining", bits_to_read)
                }
                _        => fail!("Error! Unexpected EOF!")
            }
        }
    }
}

impl<R: io::Reader> io::Reader for BitReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::IoResult<uint> {
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


#[test]
fn sample_test_function() {
}
