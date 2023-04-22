use ggez::Context;

use super::SortElement;

/// Common step interface for sorting algorithms
pub trait Sorter {
    fn step(&mut self, ctx: &Context);
    fn get_arr(&self) -> &Vec<SortElement>;
    fn is_sorted(&self) -> bool;
    fn swap_mesh(&mut self, ctx: &Context, id1: usize, id2: usize);
}
