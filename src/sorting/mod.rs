mod bubble_sort;
mod sort_element;
mod sorter;

pub use self::bubble_sort::BubbleSort;
pub use self::sort_element::SortElement;
pub use self::sorter::Sorter;

// #[cfg(test)]
// use std::cmp;

// #[cfg(test)]
// pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
// where
//     // T: cmp::PartialOrd,
//     // If HashSet is used
//     T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
// {
//     use std::collections::HashSet;

//     match a.len() == b.len() {
//         true => {
//             // This is O(n^2) but performs better on smaller data sizes
//             //b.iter().all(|item| a.contains(item))

//             // This is O(n), performs well on larger data sizes
//             let set_a: HashSet<&T> = a.iter().collect();
//             let set_b: HashSet<&T> = b.iter().collect();
//             set_a == set_b
//         }
//         false => false,
//     }
// }

// #[cfg(test)]
// pub fn is_sorted<T>(arr: &[T]) -> bool
// where
//     T: cmp::PartialOrd,
// {
//     arr.windows(2).all(|w| w[0] <= w[1])
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn is_sorted() {
//         use super::*;

//         assert!(is_sorted(&[] as &[isize]));
//         assert!(is_sorted(&["a"]));
//         assert!(is_sorted(&[1, 2, 3]));
//         assert!(is_sorted(&[0, 1, 1]));

//         assert!(!is_sorted(&[1, 0]));
//         assert!(!is_sorted(&[2, 3, 1, -1, 5]));
//     }
// }
