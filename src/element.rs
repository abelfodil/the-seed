use crate::rand::RandomValue;
use rand::Rng;

pub trait WorldElement: Default + Into<Pixel> + Copy + RandomValue {}
impl<T> WorldElement for T where T: Default + Into<Pixel> + Copy + RandomValue {}

#[derive(Clone, Copy)]
pub enum BasicElement {
    Terrain,
    Wall,
    Void,
}

impl RandomValue for BasicElement {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> BasicElement {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => BasicElement::Terrain,
            1 => BasicElement::Wall,
            2 => BasicElement::Void,
            _ => unreachable!(),
        }
    }
}

type Pixel = [u8; 4];

impl Into<Pixel> for BasicElement {
    fn into(self) -> Pixel {
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
