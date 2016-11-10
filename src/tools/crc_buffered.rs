// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms

use std::io::BufWriter;
use std::io::Write;
use std::io::Result;
use byteorder::ByteOrder;
use ::tools::CRC32;

pub struct BufWriterWithCRC<W: Write> {
    buf: BufWriter<W>,
    crc: CRC32,
}

impl<W: Write> BufWriterWithCRC<W> {
    pub fn new(inner: W) -> BufWriterWithCRC<W> {
        BufWriterWithCRC {
            buf: BufWriter::new(inner),
            crc: CRC32::new_jam(),
        }
    }

    #[inline]
    pub fn write_crc<T: ByteOrder>(&mut self) -> Result<()> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, self.crc.get_and_reset());
        self.buf.write_all(&buf)
    }

    #[inline]
    pub fn reset_crc(&mut self) {
        self.crc.reset();
    }
}

impl<W: Write> Write for BufWriterWithCRC<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let res = self.buf.write(buf);

        match &res {
            &Ok(i) => {
                for b in &buf[0..i] {
                    self.crc.add(*b);
                }
            }

            _ => {}
        }

        res
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.buf.flush()
    }
}