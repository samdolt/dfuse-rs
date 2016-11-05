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
use ::tools::BufWriterWithCRC;
use ::std::io::Result;

use ::byteorder::LittleEndian;

const CRC_SIZE: usize = 0x4;

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
    suffix: Suffix,
}

impl DfuseFile {
    /// Create an empty `DfuseFile`
    pub fn new() -> DfuseFile {
        DfuseFile {
            images: Vec::new(),
            suffix: Suffix::new(),
        }
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

        self.images.iter().fold(Prefix::size() + Suffix::size() + CRC_SIZE,
                                |sum, x| sum + x.size())
    }

    pub fn write_to<T: Write>(&self, buf: &mut T) -> Result<()> {
        let mut buf = BufWriterWithCRC::new(buf);

        let prefix = Prefix::new(self.size() as u32, self.images.len() as u8);
        try!(prefix.write_to(&mut buf));

        for image in &self.images {
            try!(image.write_to(&mut buf));
        }

        try!(suffix.write_to(&mut buf));

        // CRC is documented in the suffix section as a little endian 32bit unsigned integer
        try!(buf.write_crc::<LittleEndian>());

        try!(buf.flush());

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