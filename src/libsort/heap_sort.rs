use ggez::{
    graphics::{self, Color},
    Context,
};

use super::{SortElement, Sorter};

const NO_RECTS: usize = 150;

pub struct HeapSort {
    arr: Vec<SortElement>,
    sorted: bool,
}

impl HeapSort {
    pub fn new(ctx: &mut Context) -> Self {
        let mut sort_elems = vec![];
        for i in 0..NO_RECTS {
            sort_elems.push(SortElement::new(ctx, i).unwrap());
        }
        Self {
            sorted: false,
            arr: sort_elems,
        }
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
