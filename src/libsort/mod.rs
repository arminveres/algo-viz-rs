//! This library is a wrapper and implementation for sorting algorithms, which get visualized by
//! ggez.
mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod sort_element;
mod sorter;

pub use self::bubble_sort::BubbleSort;
pub use self::insertion_sort::InsertionSort;
pub use self::selection_sort::SelectionSort;
pub use self::sort_element::SortElement;
pub use self::sorter::Sorter;

use self::sort_element::SortState;

/// Important constants, also used for scaling the animation
pub const INIT_WINDOW_SIZE: (f32, f32) = (800., 600.);
