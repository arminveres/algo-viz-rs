use super::{sorter::swap_mesh, SortElement, Sorter};
use ggez::Context;
use rand::{self, Rng};

/// Implements the Selectionsort algorithms
///
/// The struct contains various variables that keep track of each step.
pub struct SelectionSort {
    arr: Vec<SortElement>,
    sorted: bool,
    left: usize,
    smallest: usize,
}

impl SelectionSort {
    pub fn new(ctx: &mut Context, max_value: f32, no_rects: u32) -> Self {
        let mut sort_elems = vec![];
        let mut rng = rand::thread_rng();
        for i in 0..no_rects {
            let elem_value = rng.gen_range(0.0..max_value);
            sort_elems
                .push(SortElement::new(ctx, i as usize, elem_value, max_value, no_rects).unwrap());
        }
        Self {
            sorted: false,
            arr: sort_elems,
            left: 0,
            smallest: 0,
        }
    }
}

impl Sorter for SelectionSort {
    fn step(&mut self, ctx: &Context) {
        if self.left == self.arr.len() {
            self.sorted = true;
            return;
        }
        self.smallest = self.left;
        for right in (self.left + 1)..self.arr.len() {
            if self.arr[right].get_sort_value() < self.arr[self.smallest].get_sort_value() {
                self.smallest = right;
            }
        }
        swap_mesh(&mut self.arr, ctx, self.smallest, self.left);
        self.left += 1;
    }

    fn get_arr(&mut self) -> &mut Vec<SortElement> {
        &mut self.arr
    }
    fn is_sorted(&self) -> bool {
        self.sorted
    }

    fn get_name(&self) -> &str {
        "Selectionsort"
    }
}
