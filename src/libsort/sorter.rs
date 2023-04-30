use ggez::Context;

use super::SortElement;

/// Common step interface for sorting algorithms
pub trait Sorter {
    /// One step is one sort inside a sorting algorithm in which it is implemented
    fn step(&mut self, ctx: &Context);
    /// Returns the underlying container, where the SortElements are kept
    fn get_arr(&mut self) -> &mut Vec<SortElement>;
    /// Returns whether the array is sorted
    fn is_sorted(&self) -> bool;
    /// Swaps the underlying meshes of two objects
    fn swap_mesh(&mut self, ctx: &Context, old_id: usize, new_id: usize);
}
