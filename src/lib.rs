mod sorting;

#[cfg(test)]
mod tests {
    use super::sorting;
    #[test]
    fn quick_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        sorting::heap_sort(&mut ve1);

        assert!(sorting::is_sorted(&ve1));

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        sorting::heap_sort(&mut ve2);

        assert!(sorting::is_sorted(&ve2));
    }
}
