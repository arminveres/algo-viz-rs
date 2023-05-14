use super::{SortElement, SortState, Sorter};
use ggez::Context;
use rand::{self, Rng};

/// Implements the Insertionsort algorithms
///
/// The struct contains various variables that keep track of each step.
pub struct InsertionSort {
    arr: Vec<SortElement>,
    sorted: bool,
    outer_index: usize,
    inner_index: usize,
    curr_clone: SortElement,
    do_check: bool,
}

impl InsertionSort {
    pub fn new(ctx: &mut Context, max_value: f32, no_rects: u32) -> Self {
        let mut sort_elems = vec![];
        let mut rng = rand::thread_rng();
        for i in 0..no_rects {
            let elem_value = rng.gen_range(0.0..max_value);
            sort_elems
                .push(SortElement::new(ctx, i as usize, elem_value, max_value, no_rects).unwrap());
        }
        let init_elem = sort_elems[0].clone();
        Self {
            sorted: false,
            arr: sort_elems,
            outer_index: 1,
            inner_index: 1,
            curr_clone: init_elem,
            do_check: false,
        }
    }

    /// Copies the rectangles and height of the `from_elem` to the `SortElement` at `to_id` in `self.arr`
    /// and updates `SortState`
    fn copy_rects(&mut self, to_id: usize, from_elem: SortElement) {
        let sortelems = &mut self.arr;
        let old_rect = from_elem.rect;
        sortelems[to_id].rect.h = old_rect.h;
        sortelems[to_id].sort_state = SortState::UNSORTED;
    }
}

impl Sorter for InsertionSort {
    fn step(&mut self) {
        if self.outer_index >= self.arr.len() {
            self.sorted = true;
            self.do_check = true;
            self.inner_index = 0;
            self.reset_states();
            return;
        }

        if self.inner_index > 0
            && self.curr_clone.get_sort_value() < self.arr[self.inner_index - 1].get_sort_value()
        {
            self.copy_rects(self.inner_index, self.arr[self.inner_index - 1].clone());
            self.inner_index -= 1;
            self.arr[self.inner_index].sort_state = SortState::SELECTED;
        } else {
            self.copy_rects(self.inner_index, self.curr_clone.clone());
            self.outer_index += 1;
            self.inner_index = self.outer_index;
            if self.outer_index < self.arr.len() {
                self.curr_clone = self.arr[self.outer_index].clone();
            }
        }

        if self.outer_index < self.arr.len() {
            self.arr[self.outer_index].sort_state = SortState::SORTED;
        }
    }

    fn get_arr(&mut self) -> &mut Vec<SortElement> {
        &mut self.arr
    }

    fn is_sorted(&self) -> bool {
        self.sorted
    }

    fn get_name(&self) -> &str {
        "Insertionsort"
    }

    fn do_check(&self) -> bool {
        return self.do_check;
    }

    fn check_step(&mut self) {
        self.arr[self.inner_index].sort_state = SortState::SORTED;
        self.inner_index += 1;
        if self.inner_index >= self.arr.len() {
            self.do_check = false;
            println!("Bars are sorted!");
        }
    }
}
