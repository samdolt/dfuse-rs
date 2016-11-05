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
use byteorder::WriteBytesExt;

use super::ImageElement;
use super::TargetPrefix;

pub struct Image {
    pub name: Option<String>,
    pub alternate: u8,
    pub elements: Vec<ImageElement>,
}

impl Image {
    pub fn size(&self) -> usize {
        274 + self.elements_size()
    }

    fn elements_size(&self) -> usize {
        self.elements.iter().fold(0, |sum, x| sum + x.size())
    }

    pub fn write_to<T: WriteBytesExt>(&self, buf: &mut T) -> Result<()> {
        let target = TargetPrefix::new(self.name.clone(),
                                       self.alternate,
                                       self.elements_size() as u32,
                                       self.elements.len() as u32);
        try!(target.write_to(buf));

        for element in &self.elements {
            try!(element.write_to(buf));
        }
        Ok(())
    }
}