use crate::element::WorldElement;
use crate::img::ToImage;

use image::RgbaImage;
use ndarray::{Array, Dimension, Ix2, Ix3, ShapeBuilder};
use rand::rngs::StdRng;

pub struct World<E: WorldElement, Dim: Dimension> {
    pub content: Array<E, Dim>,
}

impl<E: WorldElement, Dim: Dimension> World<E, Dim> {
    pub fn new_default(shape: impl ShapeBuilder<Dim = Dim>) -> Self {
        World {
            content: Array::default(shape),
        }
    }

    pub fn new_random(shape: impl ShapeBuilder<Dim = Dim>, rng: &mut StdRng) -> Self {
        World {
            content: Array::from_shape_simple_fn(shape, || E::random(rng)),
        }
    }
}

pub type World2D<E> = World<E, Ix2>;

impl<E: WorldElement> ToImage for World2D<E> {
    fn to_image(&self) -> RgbaImage {
        let pixels = self.content.iter().flat_map(|e| e.to_pixel()).collect();

        let (height, width) = self.content.dim();

        RgbaImage::from_raw(width as u32, height as u32, pixels)
            .expect("container should have the right size for the image dimensions")
    }
}

pub type World3D<E> = World<E, Ix3>;

pub trait ToWorld<Element: WorldElement, Dim: Dimension> {
    fn to_world(&self) -> World<Element, Dim>;
}
