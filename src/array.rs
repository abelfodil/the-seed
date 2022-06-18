use ndarray::{s, Array2};

pub trait Fillable<V> {
    fn fill_inside_area(&mut self, value: V) -> &mut Self; // assumes wall size of 1
    fn fill_outside_perimeter(&mut self, value: V) -> &mut Self; // assumes wall size of 1
}

impl<V: Copy> Fillable<V> for Array2<V> {
    fn fill_inside_area(&mut self, value: V) -> &mut Self {
        let mut slice = self.slice_mut(s![0..-1, 0..-1]);
        slice.fill(value);
        self
    }
    fn fill_outside_perimeter(&mut self, value: V) -> &mut Self {
        let mut slice = self.slice_mut(s![..;self.shape()[0]-1, ..]);
        slice.fill(value);
        let mut slice = self.slice_mut(s![.., ..;self.shape()[1]-1]);
        slice.fill(value);
        self
    }
}
