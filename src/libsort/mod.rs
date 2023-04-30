mod bubble_sort;
// mod heap_sort;
mod insertion_sort;
mod sort_element;
mod sorter;

pub use self::bubble_sort::BubbleSort;
// pub use self::heap_sort::HeapSort;
pub use self::insertion_sort::InsertionSort;
pub use self::sort_element::SortElement;
pub use self::sorter::Sorter;

/// Important constants, also used for scaling the animation
pub const INIT_WINDOW_SIZE: (f32, f32) = (800., 600.);
