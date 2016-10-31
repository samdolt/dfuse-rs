// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ::elements::*;

use ::std::io::Write;
use ::std::io::BufWriter;
use ::std::io::Result;

use ::crc::crc32::Digest;
use ::crc::crc32::IEEE;
use ::crc::Hasher32;

/// A struct representing a DFU file
///
/// # Notes
///
/// This struct store device's image on the heap
///
/// # Examples
///
/// ```
/// use dfuse::DfuseFile;
///
/// let empty_dfy = DfuseFile::new();
/// ```
pub struct DfuseFile {
    images: Vec<Image>,
}

impl DfuseFile {
    /// Create an empty `DfuseFile`
    pub fn new() -> DfuseFile {
        DfuseFile { images: Vec::new() }
    }

    /// Add a unamed binary image
    pub fn add_unamed_image(&mut self, alternate: u8, start_adress: u32, data: Vec<u8>) {
        let element = ImageElement {
            start_adress: start_adress,
            data: data,
        };

        let image = Image {
            name: None,
            alternate: alternate,
            elements: vec![element],
        };

        self.images.push(image);
    }

    pub fn size(&self) -> usize {
        self.images.iter().fold(Prefix::size() + Suffix::size(), |sum, x| sum + x.size())
    }

    pub fn write_to<T: Write>(&self, buf: &mut T) -> Result<()> {
        let mut buf = BufWriter::new(buf);
        let mut digest = Digest::new(IEEE);
        let mut data: Vec<u8> = Vec::with_capacity(self.size());

        let prefix = Prefix::new(self.size() as u32, self.images.len() as u8);
        try!(prefix.write_to(&mut data));

        for ref image in &self.images {
            try!(image.write_to(&mut data));
        }

        digest.write(&data);
        try!(buf.write_all(&data));
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_create_empty_file() {
        let file: DfuseFile = DfuseFile::new();
        drop(file);
    }
}