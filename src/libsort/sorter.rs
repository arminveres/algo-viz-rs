use super::{sort_element::SortState, SortElement, AUDIO_RANGE_HZ};
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::{sync::mpsc, thread, time};

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
    /// Returns whether a final check of all bars should be done
    fn do_check(&self) -> bool;
    /// Does one step in the final check of the bars
    fn check_step(&mut self);
    /// Resets the SortState aka. colors
    fn reset_states(&mut self) {
        for elem in self.get_arr() {
            elem.sort_state = SortState::UNSORTED;
        }
    }
}

/// Starts a separate audio thread and sets up sound output using rodio
pub fn start_audio_thread(rx: mpsc::Receiver<f32>, max_val: f32, sps: u32) {
    thread::spawn(move || {
        // Setup rodio audio parts
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        let slope = (AUDIO_RANGE_HZ.1 - AUDIO_RANGE_HZ.0) / (max_val);
        // We only leave half a frame time for the sound, in microseconds.
        // WARN: on some deviced audio might not be produced by the device drivers if one goes
        // below 10ms per sound
        let length_us = 500_000 / sps as u64;

        loop {
            // Receive data from the channel, with or without timeout, haven't noticed any
            // difference in performance yet
            // match rx.recv() {
            match rx.recv_timeout(time::Duration::from_millis(50)) {
                //Use the received data to generate the audio
                Ok(received_data) => {
                    // Normalize range to 100-1000 Hz
                    let frequency = AUDIO_RANGE_HZ.0 + (slope * (received_data));
                    let duration = time::Duration::from_micros(length_us); // Set the duration of the tone
                    let source = SineWave::new(frequency).take_duration(duration);
                    sink.append(source);
                }
                Err(_) => {
                    // Handle timeout or errors, although we don't do anything
                }
            }
        }
    });
}
