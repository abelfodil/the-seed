use crate::img::ToPixel;
use crate::random::RandomValue;
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
    fn to_pixel(&self) -> Vec<u8> {
        match self {
            BasicElement::Terrain => vec![0, 255, 0, 255],
            BasicElement::Wall => vec![0, 0, 255, 255],
            BasicElement::Void => vec![0, 0, 0, 0],
        }
    }
}

impl Default for BasicElement {
    fn default() -> BasicElement {
        BasicElement::Void
    }
}
