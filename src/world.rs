use crate::element::{WorldElement};
use image::RgbaImage;
use ndarray::{Array, Dimension, Ix2, Ix3, ShapeBuilder};
use rand::rngs::StdRng;

pub struct World<E: WorldElement, Dim: Dimension> {
    pub content: Array<E, Dim>,
}

pub type World2D<E> = World<E, Ix2>;

#[allow(dead_code)]
pub type World3D<E> = World<E, Ix3>;

impl<E: WorldElement, Dim: Dimension> World<E, Dim> {
    pub fn new_default(shape: impl ShapeBuilder<Dim = Dim>) -> Self {
        World {
            content: Array::default(shape),
        }
    }

    #[allow(dead_code)]
    pub fn new_random(shape: impl ShapeBuilder<Dim = Dim>, rng: &mut StdRng) -> Self {
        World {
            content: Array::from_shape_simple_fn(shape, || E::random(rng)),
        }
    }
}

impl<E: WorldElement> Into<RgbaImage> for World2D<E> {
    fn into(self) -> RgbaImage {
        let pixels = self.content.iter().flat_map(|e| (*e).into()).collect();

        let (height, width) = self.content.dim();

        RgbaImage::from_raw(width as u32, height as u32, pixels)
            .expect("container should have the right size for the image dimensions")
    }
}
