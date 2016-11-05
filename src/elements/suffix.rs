// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::Result;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct Suffix {
    pub fw_version: u16,
    pub usb_pid: u16,
    pub usb_vid: u16,
}

// Warning: Suffix use Little Endian
impl Suffix {
    pub fn new() -> Suffix {
        Suffix {
            fw_version: 0xFFFF,
            usb_pid: 0xFFFF,
            usb_vid: 0xFFFF,
        }
    }

    pub fn size() -> usize {
        12 // Size without CRC
    }

    pub fn write_to<T: WriteBytesExt>(&self, buf: &mut T) -> Result<()> {
        try!(buf.write_u16::<LittleEndian>(self.fw_version));
        try!(buf.write_u16::<LittleEndian>(self.usb_pid));
        try!(buf.write_u16::<LittleEndian>(self.usb_vid));

        // DFU suffix version
        try!(buf.write_u16::<LittleEndian>(0x011A));

        // DFU suffix magic number
        try!(buf.write_all(&[0x55, 0x46, 0x44]));

        // DFU suffix size with CRC
        try!(buf.write_u8(16u8));
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn assert_write_reported_size(suffix: Suffix) {
        let reported = Suffix::size();
        let mut buf: Vec<u8> = Vec::with_capacity(reported);
        suffix.write_to(&mut buf).unwrap();
        assert_eq!(reported, buf.len());
    }

    #[test]
    fn test_suffix_element_write_reported_size() {
        assert_write_reported_size(Suffix::new());
        assert_write_reported_size(Suffix {
            fw_version: 0x3344,
            usb_pid: 0x4433,
            usb_vid: 0xFF00,
        });
    }

    #[test]
    fn test_suffix_write_correct_data() {
        let suffix = Suffix {
            fw_version: 0x3344,
            usb_pid: 0x4433,
            usb_vid: 0xFF00,
        };

        let mut buf: Vec<u8> = Vec::with_capacity(Suffix::size());

        suffix.write_to(&mut buf).unwrap();

        assert_eq!(buf.len(), 12);

        let mut ptr = buf.as_mut_ptr();
        unsafe {

            // Check fw_version
            assert_eq!([0x44, 0x33], *(ptr as *const [u8; 2]));
            ptr = ptr.offset(2);

            // Check usb_pid
            assert_eq!([0x33, 0x44], *(ptr as *const [u8; 2]));
            ptr = ptr.offset(2);

            // Check usb_vid
            assert_eq!([0x00, 0xFF], *(ptr as *const [u8; 2]));
            ptr = ptr.offset(2);

            // Check DFU version
            assert_eq!([0x1A, 0x01], *(ptr as *const [u8; 2]));
            ptr = ptr.offset(2);

            // Check constant string
            assert_eq!([0x55, 0x46, 0x44], *(ptr as *const [u8; 3]));
            ptr = ptr.offset(3);

            // Check fixed length
            assert_eq!(16, *(ptr as *const u8));

        }
    }
}
