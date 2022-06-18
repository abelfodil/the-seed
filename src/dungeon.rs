use crate::array::Fillable;
use crate::element::BasicElement;
use crate::world::{ToWorld, World2D};
use ndarray::{s, Array, Array2};
use rand::{rngs::StdRng, Rng};

pub struct Room {
    location: Vec<usize>,
    content: Array2<BasicElement>,
}

impl Room {
    pub fn new(location: Vec<usize>, size: Vec<usize>) -> Self {
        let mut room = Room {
            location: location,
            content: Array::default((size[0], size[1])),
        };

        room.content
            .fill_inside_area(BasicElement::Terrain)
            .fill_outside_perimeter(BasicElement::Wall);

        room
    }
}

pub struct Dungeon {
    rng: StdRng,
    radius: usize,
    rooms: Vec<Room>,
}

impl Dungeon {
    fn size(&self) -> usize {
        self.radius * 2
    }

    fn gen_room(&mut self) -> Room {
        let half_r = self.radius / 2;

        let gen_location = |_| self.rng.gen_range(half_r..self.radius + half_r);
        let location = (0..2).into_iter().map(gen_location).collect();

        const MIN_SIZE: usize = 2;
        let gen_size = |_| self.rng.gen_range(MIN_SIZE..half_r);
        let size = (0..2).into_iter().map(gen_size).collect();

        Room::new(location, size)
    }

    pub fn new(rng: StdRng, radius: usize, n_rooms: usize) -> Self {
        let mut dungeon = Dungeon {
            rng: rng,
            radius: radius,
            rooms: vec![],
        };

        let gen_room = |_| dungeon.gen_room();
        dungeon.rooms = (0..n_rooms).into_iter().map(gen_room).collect();

        dungeon
    }

    pub fn to_world(&self) -> World2D<BasicElement> {
        let size = self.size();
        let mut world = World2D::new_default((size, size));
        for room in &self.rooms {
            let room_slice = world.content.slice_mut(s![
                room.location[0]..room.location[0] + room.content.shape()[0],
                room.location[1]..room.location[1] + room.content.shape()[1]
            ]);
            room.content.assign_to(room_slice);
        }
        world
    }
}
