use ggez::{
    graphics::{self, Color},
    Context,
};

use super::SortElement;

/// Common step interface for sorting algorithms
pub trait Sorter {
    /// The `step` function acts as a replacement for a loop, whereas one call is one iteration.
    /// The implementation should therefore consider saving variables, which get called/stored in
    /// each iteration, in the struct which implements this trait
    fn step(&mut self, ctx: &Context);
    /// Returns the underlying container, where the SortElements are kept
    fn get_arr(&mut self) -> &mut Vec<SortElement>;
    /// Returns whether the array is sorted
    fn is_sorted(&self) -> bool;
    /// Returns the name of the current sorting algorithm
    fn get_name(&self) -> &str;
}

/// Swaps the underlying meshes of two objects
pub fn swap_mesh(sortelems: &mut Vec<SortElement>, ctx: &Context, old_id: usize, new_id: usize) {
    let old_rect = sortelems[old_id].rect;
    let new_rect = sortelems[new_id].rect;
    sortelems[old_id].rect.h = new_rect.h;
    sortelems[new_id].rect.h = old_rect.h;

    // Thee coordinates of the rectangles, or all forms, be it circles, are relative, which is why
    // we use (0.0, 0.0) as x and y. The coordinates add up otherwise and will be outside of the
    // canvas onto which we are drawing

    sortelems[old_id].mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, old_rect.w, new_rect.h),
        Color::RED, // self.arr[i + 1].state.get_color(),
    )
    .unwrap();

    sortelems[new_id].mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, new_rect.w, old_rect.h),
        Color::WHITE, // self.arr[id1].state.get_color(),
    )
    .unwrap();
}
