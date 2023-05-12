# Sorting Algorithm Visualizer and Auralizer

The algorithms were adopted from [TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust)

## Algorithms

Currently the following 3 algorithms were implemented:

- [Bubblesort](./src/libsort/bubble_sort.rs)
- [Insertionsort](./src/libsort/insertion_sort.rs)
- [Selectionsort](./src/libsort/selection_sort.rs)

## Dependencies

- Rust Compiler
- All rust dependendencies will be automatically downloaded and compiled

**ATTENTION: ggez, the graphics library, requires the developmental version `0.9.0-rc0`, which works for newer compilers.
For the rust toolchain below `1.67` version `0.8.1` works.**

## TODO

- [x] Rectangle visualization
- [x] Step to step sorting update in GGEZ
- [ ] Auralization for sorting: we can use [cpal](https://github.com/RustAudio/cpal.git)
- [ ] Add tests for sortings algos
