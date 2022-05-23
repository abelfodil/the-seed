use crate::element::WorldElement;
use crate::img::ToImage;

use image::RgbaImage;
use ndarray::{Array, Dimension, Ix2, Ix3, ShapeBuilder};
use rand::{rngs::StdRng, SeedableRng};

pub struct World<E: WorldElement<E>, Dim: Dimension> {
    content: Array<E, Dim>,
    rng: StdRng,
}

impl<E: WorldElement<E>, Dim: Dimension> World<E, Dim> {
    pub fn new_default(shape: impl ShapeBuilder<Dim = Dim>, seed: u64) -> World<E, Dim> {
        World {
            content: Array::default(shape),
            rng: SeedableRng::seed_from_u64(seed),
        }
    }

    pub fn new_random(shape: impl ShapeBuilder<Dim = Dim>, seed: u64) -> World<E, Dim> {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        World {
            content: Array::from_shape_simple_fn(shape, || E::random(&mut rng)),
            rng: rng,
        }
    }

    pub fn randomize(&mut self) {
        self.content.mapv_inplace(|_| E::random(&mut self.rng));
    }
}

pub type World2D<E> = World<E, Ix2>;

impl<E: WorldElement<E>> ToImage for World2D<E> {
    fn to_image(&self) -> RgbaImage {
        let pixels: Vec<u8> = self
            .content
            .to_owned()
            .into_raw_vec()
            .iter()
            .map(|e| e.to_pixel())
            .flatten()
            .collect();

        let (height, width) = self.content.dim();

        RgbaImage::from_vec(width as u32, height as u32, pixels)
            .expect("container should have the right size for the image dimensions")
    }
}

pub type World3D<E> = World<E, Ix3>;
