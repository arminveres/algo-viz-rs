# Sorting Algorithm Visualizer and Auralizer

## Usage

```
Usage: algo-viz-rs [OPTIONS] [SORTING_ALGO]

Arguments:
  [SORTING_ALGO]  [default: bubblesort] [possible values: bubblesort, insertionsort, selectionsort]

Options:
  -m, --max-val <MAX_VAL>                    [default: 1000]
  -n, --no-rects <NO_RECTS>                  [default: 150]
  -s, --steps-per-second <STEPS_PER_SECOND>  [default: 10]
  -h, --help                                 Print help
```

## Limitations

### Audio/Auralization

To auralize the sorting steps I am using the [rodio](https://github.com/RustAudio/rodio) library.
Depending on the device and its drivers following may happen: If one goes above around 70 steps, the audio becomes
asynchronous and above 100 it disappears completely. This is because the sine waves that get generated do not have a
duration long enough for the audio device/drive on the computer to pick up and therefore they are not able to output
them.

## Algorithms

Currently the following 3 algorithms were implemented:

- [Bubblesort](./src/libsort/bubble_sort.rs)
- [Insertionsort](./src/libsort/insertion_sort.rs)
- [Selectionsort](./src/libsort/selection_sort.rs)

## Dependencies

- Rust Compiler, >=1.67

> **Warning**: ggez, the graphics library, requires the developmental version `0.9.0-rc0`, which works for newer compilers.
> For the rust toolchain below `1.67` version `0.8.1` works.

## TODO

- [ ] Add further algorithms
- [ ] Add cli option for audio range in Hz
- [ ] Once ggez 0.9.0 stable drops, update to it

## Acknowledgments

- The algorithms were adopted from [TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust)
- Used code examples from [rodio](https://github.com/RustAudio/rodio) and [ggez](https://github.com/ggez/ggez)
