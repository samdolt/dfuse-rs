use std::io::Result;
use byteorder::{BigEndian, WriteBytesExt};

pub struct Prefix {
    size: u32,
    nb_images: u8,
}

impl Prefix {
    pub fn new(size: u32, nb_images: u8) -> Prefix {
        Prefix {
            size: size,
            nb_images: nb_images,
        }
    }

    pub fn size() -> usize {
        11
    }

    pub fn write_to<T: WriteBytesExt>(&self, buf: &mut T) -> Result<()> {

        let signature: [u8; 5] = [b'D', b'f', b'u', b'S', b'e'];
        for byte in signature.iter() {
            try!(buf.write_u8(byte.clone()));
        }
        try!(buf.write_u8(0x01u8));
        try!(buf.write_u32::<BigEndian>(self.size));
        try!(buf.write_u8(self.nb_images));

        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_write_reported_size() {
        let prefix = Prefix::new(0, 0);
        let reported = Prefix::size();

        let mut buf = vec![];
        prefix.write_to(&mut buf).unwrap();
        assert_eq!(reported, buf.len());

    }

    #[test]
    fn test_prefix_write_correct_data() {
        let prefix = Prefix::new(0x00FFAA55, 0x33);
        let mut buf = vec![];
        prefix.write_to(&mut buf).unwrap();

        let mut ptr = buf.as_mut_ptr();
        unsafe {

            // Check signature
            assert_eq!(b"DfuSe", &*(ptr as *const [u8; 5]));
            ptr = ptr.offset(5);

            // Check version byte
            assert_eq!(0x01, *(ptr));
            ptr = ptr.offset(1);

            // Check size
            assert_eq!([0x00, 0xFF, 0xAA, 0x55], *(ptr as *const [u8; 4]));
            ptr = ptr.offset(4);

            // Check number of image
            assert_eq!(0x33, *(ptr));

        }

    }
}