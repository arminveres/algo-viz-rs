use super::{SortElement, SortState, Sorter};
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

    fn swap_rects(&mut self, old_id: usize, new_id: usize) {
        let old_rect = self.arr[old_id].rect;
        let new_rect = self.arr[new_id].rect;
        self.arr[old_id].rect.h = new_rect.h;
        self.arr[new_id].rect.h = old_rect.h;

        // Thee coordinates of the rectangles, or all forms, be it circles, are relative, which is why
        // we use (0.0, 0.0) as x and y. The coordinates add up otherwise and will be outside of the
        // canvas onto which we are drawing

        self.arr[old_id].sort_state = SortState::SELECTED;
        self.arr[new_id].sort_state = SortState::UNSORTED;
    }
}

impl Sorter for SelectionSort {
    fn step(&mut self) {
        if self.left == self.arr.len() {
            self.sorted = true;
            return;
        }
        self.smallest = self.left;
        for right in (self.left + 1)..self.arr.len() {
            // NOTE: important to reset sorting state for each bar, otherwise it stays red.
            self.arr[right].sort_state = SortState::UNSORTED;
            if self.arr[right].get_sort_value() < self.arr[self.smallest].get_sort_value() {
                self.smallest = right;
            }
        }
        self.swap_rects(self.smallest, self.left);
        self.arr[self.left].sort_state = SortState::SORTED;
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
