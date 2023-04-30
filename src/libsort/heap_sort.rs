use ggez::Context;
use rand::{self, Rng};

use super::{SortElement, Sorter};

pub struct HeapSort {
    arr: Vec<SortElement>,
    sorted: bool,
}

impl HeapSort {
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
        }
    }
    // --------------------------------------------------------------------------------------------
    // helper functions for heapsort
    // --------------------------------------------------------------------------------------------
    /// Convert `arr` into a max heap.
    fn heapify() {
        todo!()
    }

    /// Move the element at `root` down until `arr` is a max heap again.
    /// This assumes that the subtrees under `root` are valid max heaps already.
    fn move_down() {
        todo!()
    }
}

impl Sorter for HeapSort {
    fn step(&mut self, ctx: &Context) {
        todo!();
    }

    fn get_arr(&mut self) -> &mut Vec<SortElement> {
        &mut self.arr
    }
    fn is_sorted(&self) -> bool {
        self.sorted
    }

    fn get_name(&self) -> &str {
        "Heapsort"
    }
}
