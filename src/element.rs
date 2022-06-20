use crate::img::{ToPixel, Pixel};
use crate::rand::RandomValue;
use rand::Rng;

pub trait WorldElement: Default + ToPixel + Copy + RandomValue {}
impl<T> WorldElement for T where T: Default + ToPixel + Copy + RandomValue {}

#[derive(Clone, Copy)]
pub enum BasicElement {
    Terrain,
    Wall,
    Void,
}

impl RandomValue for BasicElement {
    fn random(rng: &mut impl Rng) -> BasicElement {
        match rng.gen_range(0..=2) as u32 {
            0 => BasicElement::Terrain,
            1 => BasicElement::Wall,
            _ => BasicElement::Void,
        }
    }
}

impl ToPixel for BasicElement {
    fn to_pixel(&self) -> Pixel {
        match self {
            BasicElement::Terrain => [0, 255, 0, 255],
            BasicElement::Wall => [0, 0, 255, 255],
            BasicElement::Void => [0, 0, 0, 0],
        }
    }
}

impl Default for BasicElement {
    fn default() -> BasicElement {
        BasicElement::Void
    }
}
