use super::{SortElement, Sorter};
use ggez::{
    graphics::{self, Color},
    Context,
};
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
                self.swap_mesh(ctx, i, i + 1);
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

    fn swap_mesh(&mut self, ctx: &Context, old_id: usize, new_id: usize) {
        let old_rect = self.arr[old_id].rect;
        let new_rect = self.arr[new_id].rect;
        self.arr[old_id].rect.h = new_rect.h;
        self.arr[new_id].rect.h = old_rect.h;

        self.arr[old_id].mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, old_rect.w, new_rect.h), // graphics::Rect::new(one.x, one.y, one.w, two.h),
            Color::WHITE, // self.arr[i + 1].state.get_color(),
        )
        .unwrap();

        self.arr[new_id].mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, new_rect.w, old_rect.h), // graphics::Rect::new(two.x, two.y, two.w, one.h),
            Color::RED, // self.arr[id1].state.get_color(),
        )
        .unwrap();

        // let mut i = 0;
        // for elem in self.arr.iter_mut() {
        //     if i == old_id || i == new_id {
        //         continue;
        //     }
        //     let old_rect = elem.rect;
        //     (*elem).mesh = graphics::Mesh::new_rectangle(
        //         ctx,
        //         graphics::DrawMode::fill(),
        //         graphics::Rect::new(0.0, 0.0, old_rect.w, old_rect.h),
        //         Color::WHITE,
        //     )
        //     .unwrap();
        //     i += 1;
        // }
    }
}
