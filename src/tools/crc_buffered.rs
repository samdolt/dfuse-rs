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
use crc::{crc32, Hasher32};
use byteorder::ByteOrder;

pub struct BufWriterWithCRC<W: Write> {
    buf: BufWriter<W>,
    digest: crc32::Digest,
}

impl<W: Write> BufWriterWithCRC<W> {
    pub fn new(inner: W) -> BufWriterWithCRC<W> {
        BufWriterWithCRC {
            buf: BufWriter::new(inner),
            digest: crc32::Digest::new(crc32::IEEE),
        }
    }

    #[inline]
    pub fn write_crc<T: ByteOrder>(&mut self) -> Result<()> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, self.digest.sum32());
        self.buf.write_all(&buf)
    }

    #[inline]
    pub fn reset_crc(&mut self) {
        self.digest.reset();
    }
}

impl<W: Write> Write for BufWriterWithCRC<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.digest.write(buf);
        self.buf.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.buf.flush()
    }
}