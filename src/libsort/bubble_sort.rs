use super::{sorter::swap_mesh, SortElement, Sorter};
use ggez::Context;
use rand::{self, Rng};

pub struct BubbleSort {
    arr: Vec<SortElement>,
    sorted: bool,
    outer_index: usize,
    inner_index: usize,
}

impl BubbleSort {
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
            outer_index: no_rects as usize,
            inner_index: 0,
        }
    }
}

impl Sorter for BubbleSort {
    fn step(&mut self, ctx: &Context) {
        // currently if there is not step left between inner and outer index, the step will be
        // empty
        if self.inner_index == 0 {
            self.sorted = true;
        }
        for i in self.inner_index..(self.outer_index - 1) {
            if self.arr[i].get_sort_value() > self.arr[i + 1].get_sort_value() {
                self.sorted = false; // if we meet another value, we obviously are unsorted
                swap_mesh(&mut self.arr, ctx, i, i + 1);
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
        self.inner_index = 0;
        return;
    }

    fn get_arr(&mut self) -> &mut Vec<SortElement> {
        &mut self.arr
    }

    fn is_sorted(&self) -> bool {
        self.sorted
    }
}
