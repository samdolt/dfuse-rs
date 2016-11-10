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
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

pub struct TargetPrefix {
    name: Option<String>,
    alternate: u8,
    image_size: u32,
    nb_elements: u32,
}

impl TargetPrefix {
    pub fn new(name: Option<String>,
               alternate: u8,
               image_size: u32,
               nb_elements: u32)
               -> TargetPrefix {
        TargetPrefix {
            name: name,
            alternate: alternate,
            image_size: image_size,
            nb_elements: nb_elements,
        }
    }

    pub fn size() -> usize {
        274
    }

    pub fn write_to<T: WriteBytesExt>(&self, buf: &mut T) -> Result<()> {
        let signature: [u8; 6] = [b'T', b'a', b'r', b'g', b'e', b't'];
        for byte in &signature {
            try!(buf.write_u8(byte.clone()));
        }
        try!(buf.write_u8(self.alternate));

        match self.name {
            Some(ref txt) => {
                // If this target is named, we should write a boolean value here
                // Spec say boolan adress is 7..11 -> u32
                try!(buf.write_u32::<BigEndian>(0x01));

                let mut i = 11;
                for c in txt.as_bytes().iter() {
                    if i < 266 {
                        try!(buf.write_u8(*c));
                        i += 1;
                    } else {
                        break;
                    }
                }
                while i < 266 {
                    try!(buf.write_u8(0u8));
                    i += 1;
                }
            }
            None => {
                // Again the same note, it's a boolean in a u32
                // It's a byte efficient file format ;)
                try!(buf.write_u32::<BigEndian>(0x00));

                for _ in 11..266 {
                    try!(buf.write_u8(0u8));
                }
            }
        }

        try!(buf.write_u32::<LittleEndian>(self.image_size));
        try!(buf.write_u32::<LittleEndian>(self.nb_elements));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_writed_size_for_name(name: Option<String>) -> usize {
        let prefix = TargetPrefix::new(name, 0xAA, 0x55555555, 0x33333333);

        let mut buf: Vec<u8> = Vec::with_capacity(TargetPrefix::size());
        prefix.write_to(&mut buf).unwrap();
        buf.len()
    }

    #[test]
    fn test_target_prefix_write_reported_size() {
        let reported = TargetPrefix::size();

        let tiny = get_writed_size_for_name(None);
        let small = get_writed_size_for_name(Some("test".to_string()));
        let big = get_writed_size_for_name(
            Some(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_string())
        );

        assert_eq!(reported, tiny);
        assert_eq!(reported, small);
        assert_eq!(reported, big);
    }

    #[test]
    #[ignore]
    fn test_target_prefix_write_correct_data() {
        let prefix = TargetPrefix::new(Some("ABCD".to_string()), 0xAB, 0x00FFFF00, 0x3355AA00);
        let mut buf = vec![];
        prefix.write_to(&mut buf).unwrap();

        let mut ptr = buf.as_mut_ptr();
        unsafe {

            // Check signature
            assert_eq!(b"Target", &*(ptr as *const [u8; 6]));
            ptr = ptr.offset(6);

            // Check alternate byte
            assert_eq!(0xAB, *(ptr));
            ptr = ptr.offset(1);

            // Check this 32bit boolean
            assert_eq!([0x00, 0x00, 0x00, 0x01], *(ptr as *const [u8; 4]));
            ptr = ptr.offset(4);

            // Check the name
            assert_eq!([b'A', b'B', b'C', b'D', 0x0], *(ptr as *const [u8; 5]));
            ptr = ptr.offset(255);

            // Check the size
            assert_eq!([0x00, 0xFF, 0xFF, 0x00], *(ptr as *const [u8; 4]));
            ptr = ptr.offset(4);

            // Check the number of image_size
            assert_eq!([0x33, 0x55, 0xAA, 0x00], *(ptr as *const [u8; 4]));
        }

    }
}
