//! This library is a wrapper and implementation for sorting algorithms, which get visualized by
//! ggez.
mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod sort_element;
mod sorter;

// Public exports
pub use self::bubble_sort::BubbleSort;
pub use self::insertion_sort::InsertionSort;
pub use self::selection_sort::SelectionSort;
pub use self::sort_element::SortElement;
pub use self::sorter::Sorter;
pub use self::sorter::start_audio_thread;

// Protected exports
use self::sort_element::SortState;

/// Important constants, also used for scaling the animation
pub const INIT_WINDOW_SIZE: (f32, f32) = (800., 600.);

/// Range in between which the audio beep is played
/// TODO: could make this part of the cli
pub const AUDIO_RANGE_HZ: (f32, f32) = (120., 1212.);
