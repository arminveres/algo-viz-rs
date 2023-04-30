use ggez::{
    graphics::{self, Color},
    Context,
};

use super::SortElement;

/// Common step interface for sorting algorithms
pub trait Sorter {
    /// One step is one sort inside a sorting algorithm in which it is implemented
    fn step(&mut self, ctx: &Context);
    /// Returns the underlying container, where the SortElements are kept
    fn get_arr(&mut self) -> &mut Vec<SortElement>;
    /// Returns whether the array is sorted
    fn is_sorted(&self) -> bool;
}

/// Swaps the underlying meshes of two objects
pub fn swap_mesh(sortelemes: &mut Vec<SortElement>, ctx: &Context, old_id: usize, new_id: usize) {
    let old_rect = sortelemes[old_id].rect;
    let new_rect = sortelemes[new_id].rect;
    sortelemes[old_id].rect.h = new_rect.h;
    sortelemes[new_id].rect.h = old_rect.h;

    sortelemes[old_id].mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, old_rect.w, new_rect.h), // graphics::Rect::new(one.x, one.y, one.w, two.h),
        Color::WHITE,                                          // self.arr[i + 1].state.get_color(),
    )
    .unwrap();

    sortelemes[new_id].mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, new_rect.w, old_rect.h), // graphics::Rect::new(two.x, two.y, two.w, one.h),
        Color::RED,                                            // self.arr[id1].state.get_color(),
    )
    .unwrap();
}
