use super::{sort_element::SortState, SortElement};

/// Common step interface for sorting algorithms
pub trait Sorter {
    /// The `step` function acts as a replacement for a loop, whereas one call is one iteration.
    /// The implementation should therefore consider saving variables, which get called/stored in
    /// each iteration, in the struct which implements this trait
    fn step(&mut self);
    /// Returns the underlying container, where the SortElements are kept
    fn get_arr(&mut self) -> &mut Vec<SortElement>;
    /// Returns whether the array is sorted
    fn is_sorted(&self) -> bool;
    /// Returns the name of the current sorting algorithm
    fn get_name(&self) -> &str;
    /// Resets the colors
    fn reset_states(&mut self) {
        for elem in self.get_arr() {
            elem.sort_state = SortState::UNSORTED;
        }
    }
    fn do_check(&self) -> bool;
    fn check_step(&mut self);
}
