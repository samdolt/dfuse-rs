mod prefix;
pub use self::prefix::Prefix;

mod target_prefix;
pub use self::target_prefix::TargetPrefix;

mod image_element;
pub use self::image_element::ImageElement;

pub struct Suffix;

impl Suffix {
    pub fn size() -> usize {
        16
    }
}


pub struct Image {
    pub name: Option<String>,
    pub alternate: u8,
    pub elements: Vec<ImageElement>,
}

impl Image {
    pub fn size(&self) -> usize {
        self.elements.iter().fold(274, |sum, x| sum + x.size())
    }
}
