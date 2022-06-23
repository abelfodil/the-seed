use crate::array::{Cast, Fill, Normalize};
use crate::element::BasicElement;
use crate::num::Round;
use crate::world::{ToWorld, World2D};
use ndarray::{s, Array, Array1, Array2};
use rand::{rngs::StdRng, Rng};
use std::{cmp::Ord, ops::Neg};

#[derive(Clone)]
pub struct Room {
    location: Array1<usize>,
    dims: Array1<usize>,
    content: Array2<BasicElement>,
    dungeon_size: usize,
}

impl Room {
    pub fn new(location: Array1<usize>, size: Vec<usize>, dungeon_size: usize) -> Self {
        let mut room = Room {
            location: location,
            dims: Array1::from_vec(size.clone()),
            content: Array::default((size[0], size[1])),
            dungeon_size: dungeon_size,
        };

        room.content
            .fill_inside_area(BasicElement::Terrain)
            .fill_outside_perimeter(BasicElement::Wall);

        room
    }

    pub fn new_random(rng: &mut StdRng, dungeon_size: usize) -> Self {
        let size = Room::gen_size(rng, dungeon_size);
        let location = Room::gen_location(rng, dungeon_size, &size);
        Room::new(location, size.to_vec(), dungeon_size)
    }

    fn gen_size(rng: &mut StdRng, dungeon_size: usize) -> Array1<usize> {
        let half_r = dungeon_size / 4;
        const MIN_SIZE: usize = 10;
        let gen_size = |_| rng.gen_range(MIN_SIZE..half_r / 2);
        Array::from_iter((0..2).into_iter().map(gen_size))
    }

    fn gen_location(rng: &mut StdRng, dungeon_size: usize, size: &Array1<usize>) -> Array1<usize> {
        let half_r = dungeon_size / 4;
        let gen_location = |_| rng.gen_range(0..half_r);
        let middle_location = Array::from_iter((0..2).into_iter().map(gen_location));
        let top_right_location = middle_location - size / 2 + half_r * 3 / 2;
        top_right_location
    }

    // https://www.hackerearth.com/practice/notes/how-to-check-if-two-rectangles-intersect-or-not/
    pub fn is_overlapping(&self, other: &Room) -> bool {
        let l1 = self.top_left();
        let r1 = self.bottom_right();

        let l2 = other.top_left();
        let r2 = other.bottom_right();

        !((l1[0] > r2[0]) || (r1[0] < l2[0]) || (l1[1] > r2[1]) || (r1[1] < l2[1]))
    }

    fn top_left(&self) -> Array1<usize> {
        self.location.clone()
    }

    fn bottom_right(&self) -> Array1<usize> {
        self.top_left() + self.dims.clone()
    }

    pub fn middle(&self) -> Array1<f32> {
        self.top_left().to::<f32>() + self.dims.to::<f32>() / 2.
    }

    pub fn translate(&mut self, direction: &Array1<i32>) -> &mut Self {
        let clip = |e: &i32| (*e).clamp(0, self.dungeon_size as i32) as usize;
        self.location = (self.location.map(|e| *e as i32) + direction).map(clip);
        self
    }

    pub fn translate_direction(room1: &Room, room2: &Room) -> Array1<i32> {
        (room2.middle() - room1.middle())
            .normalize()
            .map(|e| (*e).signed_ceil() as i32)
    }
}

pub struct Dungeon {
    size: usize,
    rooms: Vec<Room>,
}

impl Dungeon {
    pub fn new(rng: &mut StdRng, size: usize) -> Self {
        let gen_room = |_| Room::new_random(rng, size);
        let n_rooms = (size as f64).sqrt() as usize;
        let mut dungeon = Dungeon {
            size: size,
            rooms: (0..n_rooms).into_iter().map(gen_room).collect(),
        };
        dungeon.separate_rooms();
        dungeon
    }

    fn separate_rooms(&mut self) -> &mut Self {
        let mut any_room_overlapping = true;
        while any_room_overlapping {
            any_room_overlapping = false;
            for i in 0..self.rooms.len() {
                for j in 0..self.rooms.len() {
                    if i == j {
                        continue;
                    }

                    let direction: Option<Array1<i32>> = {
                        let room1 = &self.rooms[i];
                        let room2 = &self.rooms[j];
                        if room1.is_overlapping(&room2) {
                            Some(Room::translate_direction(room1, room2))
                        } else {
                            None
                        }
                    };

                    if direction.is_none() {
                        continue;
                    }

                    any_room_overlapping = true;
                    let direction = direction.as_ref().unwrap();
                    self.rooms[i].translate(&direction.neg());
                    self.rooms[j].translate(direction);
                }
            }
        }

        self
    }

    pub fn to_world(&self) -> World2D<BasicElement> {
        let mut world = World2D::new_default((self.size, self.size));
        for room in &self.rooms {
            let top_left = room.top_left();
            let bottom_right = room.bottom_right();
            let room_slice = world.content.slice_mut(s![
                top_left[0]..bottom_right[0],
                top_left[1]..bottom_right[1]
            ]);
            room.content.assign_to(room_slice);
        }
        world
    }
}
