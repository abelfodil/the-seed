use crate::array::{Cast, Normalize};
use crate::num::Round;
use delaunator::{triangulate, Point};
use ndarray::{arr1, Array1};
use petgraph::data::Element;
use std::ops::Neg;

pub trait Area {
    fn area(&self) -> usize;
}

pub trait Overlap {
    fn is_overlapping(&self, other: &Self) -> bool;
}

pub trait Middle: Sized {
    fn middle(&self) -> Array1<f64>;
    fn middles(objects: &[Self]) -> Vec<Array1<f64>> {
        objects.iter().map(|e| e.middle()).collect()
    }
}

pub trait Triangulate: Middle {
    fn triangulate(objects: &[Self]) -> Vec<Element<Self, usize>> {
        let middles: Vec<_> = Self::middles(objects)
            .iter()
            .map(|e| Point { x: e[0], y: e[1] })
            .collect();

        let triangles = triangulate(&middles).triangles;

        triangles
            .windows(2)
            .map(|window| Element::Edge {
                source: window[0],
                target: window[1],
                weight: 0,
            })
            .collect()
    }
}

pub trait Translate
where
    Self: Middle,
{
    fn translate(&mut self, direction: &Array1<i32>) -> &mut Self;
    fn translate_direction(obj1: &Self, obj2: &Self) -> Array1<f64> {
        obj2.middle() - obj1.middle()
    }
    fn normalized_translate_direction(obj1: &Self, obj2: &Self) -> Array1<i32> {
        Self::translate_direction(obj1, obj2)
            .normalize()
            .map(|e| (*e).signed_ceil() as i32)
    }
}

pub trait Separate
where
    Self: Translate + Overlap + Sized,
{
    fn separate(objects: &mut [Self]) {
        let mut any_object_overlapping = true;
        while any_object_overlapping {
            any_object_overlapping = false;
            for i in 0..objects.len() {
                for j in 0..objects.len() {
                    if i == j {
                        continue;
                    }

                    let direction: Option<Array1<i32>> = {
                        let object1 = &objects[i];
                        let object2 = &objects[j];
                        if object1.is_overlapping(&object2) {
                            Some(Self::normalized_translate_direction(object1, object2))
                        } else {
                            None
                        }
                    };

                    if direction.is_none() {
                        continue;
                    }

                    any_object_overlapping = true;
                    let direction = direction.as_ref().unwrap();
                    objects[i].translate(&direction.neg());
                    objects[j].translate(direction);
                }
            }
        }
    }
}

pub trait Rectangle: Overlap + Middle + Area + Translate {
    fn top_left(&self) -> Array1<usize>;
    fn bottom_right(&self) -> Array1<usize>;
    fn dimensions(&self) -> Array1<usize>;
    fn get_edge_location(&self, direction: Array1<f64>) -> Array1<usize> {
        let angle = direction[1].atan2(direction[0]);
        let quadrant = ((angle + std::f64::consts::FRAC_PI_4 + std::f64::consts::TAU)
            / std::f64::consts::FRAC_PI_2)
            .floor() as u32
            % 4;
        let middle = self.middle();
        let half_dims = self.dimensions().to::<f64>() / 2.;
        let to_int = |arr: Array1<f64>| arr.map(|e| e.floor() as usize);
        match quadrant {
            0 => to_int(middle + arr1(&[half_dims[0], 0.])),
            1 => to_int(middle + arr1(&[0., half_dims[1]])),
            2 => to_int(middle + arr1(&[-half_dims[0], 0.])),
            3 => to_int(middle + arr1(&[0., -half_dims[1]])),
            _ => unreachable!(),
        }
    }
}

impl<T: Rectangle> Overlap for T {
    // https://www.hackerearth.com/practice/notes/how-to-check-if-two-rectangles-intersect-or-not/
    fn is_overlapping(&self, other: &Self) -> bool {
        let l1 = self.top_left();
        let r1 = self.bottom_right();

        let l2 = other.top_left();
        let r2 = other.bottom_right();

        !((l1[0] > r2[0]) || (r1[0] < l2[0]) || (l1[1] > r2[1]) || (r1[1] < l2[1]))
    }
}

impl<T: Rectangle> Middle for T {
    fn middle(&self) -> Array1<f64> {
        self.top_left().to::<f64>() + self.dimensions().to::<f64>() / 2.
    }
}

impl<T: Rectangle> Area for T {
    fn area(&self) -> usize {
        self.dimensions().product()
    }
}

impl<T: Translate + Overlap + Sized> Separate for T {}

impl<T: Middle> Triangulate for T {}
