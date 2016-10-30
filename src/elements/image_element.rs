use std::io::Result;
use byteorder::{BigEndian, WriteBytesExt};

pub struct ImageElement {
    pub start_adress: u32,
    pub data: Vec<u8>,
}

impl ImageElement {
    pub fn new(start_adress: u32, data: Vec<u8>) -> ImageElement {
        ImageElement {
            start_adress: start_adress,
            data: data,
        }
    }
    pub fn size(&self) -> usize {
        8 + self.data.len()
    }

    pub fn write_to<T: WriteBytesExt>(&self, buf: &mut T) -> Result<()> {
        try!(buf.write_u32::<BigEndian>(self.start_adress));
        try!(buf.write_u32::<BigEndian>(self.data.len() as u32));

        for b in self.data.as_slice().iter() {
            try!(buf.write_u8(*b));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_write_reported_size(data: Vec<u8>) {
        let element = ImageElement::new(0xAABBCCDDEE, data);
        let reported = element.size();
        let mut buf: Vec<u8> = Vec::with_capacity(reported);
        element.write_to(&mut buf).unwrap();
        assert_eq!(reported, buf.len());
    }

    #[test]
    fn test_image_element_write_reported_size() {
        assert_write_reported_size(vec![0x01, 0x02, 0x03, 0x04]);
        assert_write_reported_size(vec![0x03, 0x06, 0x08, 0x09]);
    }

    #[test]
    fn test_image_element_write_correct_data() {
        let element = ImageElement::new(0x008CFFFF, vec![0x33, 0x44, 0x55]);
        let mut buf: Vec<u8> = Vec::with_capacity(element.size());

        element.write_to(&mut buf).unwrap();

        assert_eq!(buf.len(), 11);

        let mut ptr = buf.as_mut_ptr();
        unsafe {

            // Check adress
            assert_eq!([0x00, 0x8C, 0xFF, 0xFF], *(ptr as *const [u8; 4]));
            ptr = ptr.offset(4);

            // Check size
            assert_eq!([0x00, 0x00, 0x00, 0x03], *(ptr as *const [u8; 4]));
            ptr = ptr.offset(4);

            // Check data
            assert_eq!([0x33, 0x44, 0x55], *(ptr as *const [u8; 3]));
        }
    }
}