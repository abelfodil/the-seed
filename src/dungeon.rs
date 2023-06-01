use crate::array::Cast;
use crate::element::BasicElement;
use crate::geom::{Area, Rectangle, Separate, Translate, Triangulate};
use crate::room::Room;
use crate::world::World2D;
use ndarray::s;
use petgraph::{algo::min_spanning_tree, data::FromElements, graph::UnGraph};
use rand::Rng;
use std::cmp::{max, min};
use std::convert::Into;

pub struct Dungeon {
    rooms: UnGraph<Room, usize>,
    size: usize,
}

impl Dungeon {
    pub fn new<R: Rng + ?Sized>(rng: &mut R, size: usize) -> Self {
        let gen_room = |_| Room::new_random(rng, size);
        let n_rooms = (size as f64).sqrt() as usize;
        let mut raw_rooms: Vec<_> = (0..n_rooms).into_iter().map(gen_room).collect();
        Room::separate(&mut raw_rooms);
        raw_rooms = Self::filter_rooms(size, raw_rooms);
        let edges = Room::triangulate(&raw_rooms);
        let raw_rooms = Room::to_elements(raw_rooms);
        let rooms = UnGraph::<Room, usize>::from_elements(raw_rooms.chain(edges.into_iter()));
        let rooms = UnGraph::<Room, usize>::from_elements(min_spanning_tree(&rooms));
        Self { size, rooms }
    }

    fn raw_rooms(&self) -> impl Iterator<Item = &Room> + '_ {
        self.rooms.raw_nodes().iter().map(|e| &e.weight)
    }

    fn filter_rooms(size: usize, rooms: Vec<Room>) -> Vec<Room> {
        rooms
            .into_iter()
            .filter(|room| {
                let area = room.area();
                let lower_bound = (size / 12).pow(2);
                let upper_bound = (size / 8).pow(2);
                return area > lower_bound && area < upper_bound;
            })
            .collect()
    }
}

impl Into<World2D<BasicElement>> for Dungeon {
    fn into(self) -> World2D<BasicElement> {
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

        for index in self.rooms.edge_indices() {
            let (room_index1, room_index2) = self.rooms.edge_endpoints(index).unwrap();
            let (room1, room2) = (&self.rooms[room_index1], &self.rooms[room_index2]);
            let direction = Room::translate_direction(room1, room2);
            let exit1 = room1.get_edge_location(direction.clone());
            let exit2 = room2.get_edge_location(-direction.clone());
            let direction = exit1.to::<i32>() - exit2.to::<i32>();
            const THRESHHOLD: i32 = 10;
            let abs_direction = direction.map(|x| x.abs());
            if abs_direction[0] < THRESHHOLD {
                world
                    .content
                    .slice_mut(s![exit1[0], exit1[1].min(exit2[1])..exit1[1].max(exit2[1])])
                    .fill(BasicElement::Path);
            } else if abs_direction[1] < THRESHHOLD {
                world
                    .content
                    .slice_mut(s![exit1[0].min(exit2[0])..exit1[0].max(exit2[0]), exit1[1]])
                    .fill(BasicElement::Path);
            } else {
                let pos1 = exit2[0] + direction[0];
                let pos2 = exit2[0];
                world
                    .content
                    .slice_mut(s![pos1.min(pos2)..pos1.max(pos2), exit2[1]])
                    .fill(BasicElement::Path);

                let pos1 = exit1[1] - direction[1];
                let pos2 = exit1[1];
                world
                    .content
                    .slice_mut(s![exit1[0], pos1.min(pos2)..pos1.max(pos2)])
                    .fill(BasicElement::Path);
            }
        }

        world
    }
}
