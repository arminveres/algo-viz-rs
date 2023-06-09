use super::{sort_element::SortState, SortElement, Sorter};
use ggez::Context;
use rand::{self, Rng};
use std::sync::mpsc;

/// Implements the Bubblesort algorithms
///
/// The struct contains various variables that keep track of each step.
pub struct BubbleSort {
    arr: Vec<SortElement>,
    sorted: bool,
    outer_index: usize,
    /// Inner loop index, later also used to check if sorted
    inner_index: usize,
    do_check: bool,
    tx: mpsc::Sender<f32>,
}

impl BubbleSort {
    pub fn new(ctx: &mut Context, max_value: f32, no_rects: u32, tx: mpsc::Sender<f32>) -> Self {
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
            outer_index: no_rects as usize,
            inner_index: 0,
            do_check: false,
            tx,
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

        self.tx.send(self.arr[old_id].get_sort_value()).unwrap();
        self.arr[old_id].sort_state = SortState::UNSORTED;
        // self.tx.send(self.arr[new_id].get_sort_value()).unwrap();
        self.arr[new_id].sort_state = SortState::SELECTED;
    }
}

impl Sorter for BubbleSort {
    fn step(&mut self) {
        // currently if there is not step left between inner and outer index, the step will be
        // empty
        if self.outer_index == 0 {
            self.sorted = true;
            self.do_check = true;
            self.inner_index = 0;
            self.reset_states();
            return;
        }
        for i in self.inner_index..(self.outer_index - 1) {
            // NOTE: important to reset sorting state for each bar, otherwise it stays red.
            self.arr[i].sort_state = SortState::UNSORTED;
            if self.arr[i].get_sort_value() > self.arr[i + 1].get_sort_value() {
                self.sorted = false; // if we meet another value, we obviously are unsorted
                self.swap_rects(i, i + 1);
                if i < self.outer_index - 1 {
                    self.inner_index = i;
                } else {
                    self.outer_index -= 1;
                    self.inner_index = 0;
                }
                return;
            }
        }
        self.outer_index -= 1;
        // the last bar is already sorted so indicate that
        // self.tx
        //     .send(self.arr[self.outer_index].get_sort_value())
        //     .unwrap();
        self.arr[self.outer_index].sort_state = SortState::SORTED;
        self.inner_index = 0;
        return;
    }

    fn get_arr(&mut self) -> &mut Vec<SortElement> {
        &mut self.arr
    }

    fn is_sorted(&self) -> bool {
        self.sorted
    }

    fn get_name(&self) -> &str {
        "Bubblesort"
    }

    fn do_check(&self) -> bool {
        return self.do_check;
    }

    fn check_step(&mut self) {
        if self.arr[self.inner_index].get_sort_value()
            <= self.arr[self.inner_index + 1].get_sort_value()
        {
            self.tx
                .send(self.arr[self.inner_index].get_sort_value())
                .unwrap();
            self.arr[self.inner_index].sort_state = SortState::SORTED;
            self.inner_index += 1;
            if self.inner_index >= self.arr.len() - 1 {
                self.arr[self.inner_index].sort_state = SortState::SORTED;
                self.do_check = false;
                println!("Bars are sorted!");
            }
        }
    }
}
