// Copyright © 2016 - Samuel Dolt <samuel@dolt.ch>
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod prefix;
pub use self::prefix::Prefix;

mod target_prefix;
pub use self::target_prefix::TargetPrefix;

mod image_element;
pub use self::image_element::ImageElement;

mod image;
pub use self::image::Image;

pub struct Suffix;

// Warning: Suffix use Little Endian
impl Suffix {
    pub fn size() -> usize {
        16
    }
}
