use crate::element::BasicElement;
use crate::geom::{Rectangle, Separate, Triangulate};
use crate::room::Room;
use crate::world::{ToWorld, World2D};
use ndarray::s;
use petgraph::{
    data::{Element, FromElements},
    graph::UnGraph,
};
use rand::rngs::StdRng;

pub struct Dungeon {
    size: usize,
    rooms: UnGraph<Room, usize>,
}

impl Dungeon {
    pub fn new(rng: &mut StdRng, size: usize) -> Self {
        let gen_room = |_| Room::new_random(rng, size);
        let n_rooms = (size as f64).sqrt() as usize;
        let mut raw_rooms: Vec<_> = (0..n_rooms).into_iter().map(gen_room).collect();
        Room::separate(&mut raw_rooms);
        let edges = Room::triangulate(&raw_rooms);
        let raw_rooms: Vec<_> = Dungeon::rooms_to_elements(raw_rooms);
        let rooms =
            UnGraph::<Room, usize>::from_elements(raw_rooms.into_iter().chain(edges.into_iter()));
        let dungeon = Dungeon { size, rooms };
        dungeon
    }

    fn raw_rooms(&self) -> impl Iterator<Item = &Room> + '_ {
        self.rooms.raw_nodes().iter().map(|e| &e.weight)
    }

    fn rooms_to_elements(rooms: Vec<Room>) -> Vec<Element<Room, usize>> {
        rooms
            .into_iter()
            .map(|room| Element::Node { weight: room })
            .collect()
    }

    pub fn to_world(&self) -> World2D<BasicElement> {
        let mut world = World2D::new_default((self.size, self.size));
        for room in self.raw_rooms() {
            let top_left = room.top_left();
            let bottom_right = room.bottom_right();
            let world_slice = world.content.slice_mut(s![
                top_left[0]..bottom_right[0],
                top_left[1]..bottom_right[1]
            ]);
            room.content.assign_to(world_slice);
        }
        world
    }
}
