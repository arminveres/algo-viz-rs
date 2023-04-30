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

    fn get_arr(&self) -> &Vec<SortElement> {
        &self.arr
    }

    fn is_sorted(&self) -> bool {
        self.sorted
    }

    fn swap_mesh(&mut self, ctx: &Context, id1: usize, id2: usize) {
        let one = self.arr[id1].rect;
        let two = self.arr[id2].rect;
        self.arr[id1].rect.h = two.h;
        self.arr[id2].rect.h = one.h;

        self.arr[id1].mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(one.x, one.y, one.w, two.h),
            // self.arr[i + 1].state.get_color(),
            Color::GREEN,
        )
        .unwrap();

        self.arr[id2].mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(two.x, two.y, two.w, one.h),
            self.arr[id1].state.get_color(),
        )
        .unwrap();
    }
}
