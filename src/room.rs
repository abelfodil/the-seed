use crate::array::Fill;
use crate::element::BasicElement;
use crate::geom::{Rectangle, Translate};
use ndarray::{Array, Array1, Array2};
use petgraph::data::Element;
use rand::Rng;
use std::cmp::Ord;

#[derive(Clone)]
pub struct Room {
    pub content: Array2<BasicElement>,
    location: Array1<usize>,
    dungeon_size: usize,
}

impl Room {
    pub fn new(location: Array1<usize>, size: Vec<usize>, dungeon_size: usize) -> Self {
        Room {
            location: location,
            content: Array::default((size[0], size[1]))
                .fill_inside_area(BasicElement::Terrain)
                .fill_outside_perimeter(BasicElement::Wall)
                .clone(),
            dungeon_size: dungeon_size,
        }
    }

    pub fn new_random<R: Rng + ?Sized>(rng: &mut R, dungeon_size: usize) -> Self {
        let size = Room::gen_size(rng, dungeon_size);
        let location = Room::gen_location(rng, dungeon_size, &size);
        Room::new(location, size.to_vec(), dungeon_size)
    }

    pub fn to_elements(rooms: Vec<Room>) -> impl Iterator<Item = Element<Room, usize>> {
        rooms.into_iter().map(|room| room.into())
    }

    fn gen_size<R: Rng + ?Sized>(rng: &mut R, dungeon_size: usize) -> Array1<usize> {
        let half_r = dungeon_size / 4;
        const MIN_SIZE: usize = 10;
        let gen_size = |_| rng.gen_range(MIN_SIZE..half_r / 2);
        Array::from_iter((0..2).into_iter().map(gen_size))
    }

    fn gen_location<R: Rng + ?Sized>(
        rng: &mut R,
        dungeon_size: usize,
        size: &Array1<usize>,
    ) -> Array1<usize> {
        let half_r = dungeon_size / 4;
        let gen_location = |_| rng.gen_range(0..half_r);
        let middle_location = Array::from_iter((0..2).into_iter().map(gen_location));
        let top_right_location = (middle_location + half_r * 3 / 2) - size / 2;
        top_right_location
    }
}

impl Rectangle for Room {
    fn dimensions(&self) -> Array1<usize> {
        Array1::from_iter(self.content.shape().to_vec())
    }

    fn top_left(&self) -> Array1<usize> {
        self.location.clone()
    }

    fn bottom_right(&self) -> Array1<usize> {
        self.top_left() + self.dimensions().clone()
    }
}

impl Translate for Room {
    fn translate(&mut self, direction: &Array1<i32>) -> &mut Self {
        let clip = |e: &i32| (*e).clamp(0, self.dungeon_size as i32) as usize;
        self.location = (self.location.map(|e| *e as i32) + direction).map(clip);
        self
    }
}

impl Into<Element<Room, usize>> for Room {
    fn into(self) -> Element<Room, usize> {
        Element::Node { weight: self }
    }
}
