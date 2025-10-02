//! Represents a musical or timed sequence composed of multiple concurrent lines.

use crate::log_eprintln;
use serde::{Deserialize, Serialize};
use std::usize;
mod line;
pub mod script;
mod frame;

pub use line::Line;
pub use frame::Frame;

/// Default speed factor for lines if not specified.
/// Returns `1.0`. Used for serde default.
pub fn default_speed_factor() -> f64 {
    1.0f64
}

/// Represents a scene, which is a collection of [`Line`]s that can play concurrently.
///
/// A scene defines the overall structure and timing for a musical piece or timed sequence.
/// It primarily holds a vector of `Line` objects, each representing a distinct track or sequence
/// of events (frames) with associated scripts.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// The collection of lines that make up this scene.
    /// Each `Line` runs concurrently within the scene's context.
    pub lines: Vec<Line>,
}

impl Scene {
    /// Creates a new `Scene` with the given lines.
    ///
    /// Initializes the `index` field of each provided `Line` according to its position
    /// in the input vector. Sets a default `length` (currently hardcoded to 4).
    pub fn new(mut lines: Vec<Line>) -> Self {
        for (i, s) in lines.iter_mut().enumerate() {
            s.index = i;
        }
        Scene { lines }
    }

    /// Ensures the consistency of the scene and all its contained lines.
    ///
    /// Iterates through each `Line` in the scene, ensuring its `index` is correct
    /// and calling the `make_consistent` method on each line to synchronize its internal state
    /// (e.g., frame counts, script indices, vector lengths).
    pub fn make_consistent(&mut self) {
        for (i, s) in self.lines.iter_mut().enumerate() {
            s.index = i;
            s.make_consistent();
        }
    }

    pub fn get_frame(&self, line_id: usize, frame_id: usize) -> Option<&Frame> {
        self.line(line_id).and_then(|line| line.frame(frame_id))
    }

    pub fn get_frame_mut(&mut self, line_id: usize, frame_id: usize) -> &mut Frame {
        self.line_mut(line_id).frame_mut(frame_id)
    }

    /// Returns the number of lines currently in the scene.
    #[inline]
    pub fn n_lines(&self) -> usize {
        self.lines.len()
    }

    /// Returns an iterator over immutable references to the lines in the scene.
    pub fn lines_iter(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    /// Returns an iterator over mutable references to the lines in the scene.
    pub fn lines_iter_mut(&mut self) -> impl Iterator<Item = &mut Line> {
        self.lines.iter_mut()
    }

    /// Returns an immutable slice containing all lines in the scene.
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    /// Returns a mutable slice containing all lines in the scene.
    pub fn line_muts(&mut self) -> &mut [Line] {
        &mut self.lines
    }

    /// Adds a new line to the end of the scene.
    ///
    /// Sets the `index` of the provided `line` to the next available index (current number of lines),
    /// ensures the line is internally consistent via `make_consistent`, and then appends it to the `lines` vector.
    pub fn add_line(&mut self, mut line: Line) {
        line.index = self.n_lines();
        line.make_consistent();
        self.lines.push(line);
    }

    /// Inserts a new line to the end of the scene.
    ///
    /// Sets the `index` of the provided `line` to the next available index (current number of lines),
    /// ensures the line is internally consistent via `make_consistent`, and then appends it to the `lines` vector.
    pub fn insert_line(&mut self, at: usize, line: Line) {
        self.ensure_min_size(at);
        self.lines.insert(at, line);
        self.make_consistent();
    }

    /// Replaces the line at the specified `index` with the provided `line`.
    ///
    /// Handles wrapping: if `index` is out of bounds, it creates intermediary lines with default value.
    /// Sets the `index` field of the new `line` correctly, calls `make_consistent` on it, and places it at the target index.
    /// Prints a warning and does nothing if the scene is empty.
    pub fn set_line(&mut self, index: usize, line: Line) {
        if self.n_lines() <= index {
            self.lines.resize(index + 1, Line::default());
        }
        self.lines[index] = line;
        self.make_consistent();
    }

    /// Removes the line at the specified `index` from the scene.
    /// 
    /// After removing the line, it updates the `index` field of all subsequent lines to maintain correct sequential indices.
    /// Prints a warning and does nothing if the scene is empty.
    pub fn remove_line(&mut self, index: usize) {
        if index >= self.n_lines() {
            log_eprintln!(
                "Warning: Attempted to remove line with invalid index {}. Ignoring.",
                index
            );
            return;
        }
        self.lines.remove(index);
        for (i, line) in self.lines[index..].iter_mut().enumerate() {
            line.index = index + i;
        }
    }

    /// Returns an immutable reference to the line at the specified `index`,
    /// or None if it doesn't exist.
    pub fn line(&self, index: usize) -> Option<&Line> {
        if index >= self.n_lines() {
            log_eprintln!(
                "Warning: Attempted to get line with invalid index {}. Ignoring.",
                index
            );
            return None;
        }
        Some(&self.lines[index])
    }

    /// Returns a mutable reference to the line at the specified `index`.
    ///
    /// Handles wrapping: if `index` is out of bounds, it creates intermediary lines.
    pub fn line_mut(&mut self, index: usize) -> &mut Line {
        if index >= self.n_lines() {
            self.lines.resize(index + 1, Line::default());
            self.make_consistent();
        }
        &mut self.lines[index]
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn ensure_min_size(&mut self, size: usize) {
        if self.n_lines() < size {
            self.lines.resize(size, Line::default());
        }
    }

    /// Collects the `current_frame` index from each line in the scene.
    ///
    /// Useful for getting a snapshot of the playback position of all lines.
    pub fn get_frames_positions(&self) -> Vec<usize> {
        self.lines_iter().map(|s| s.current_frame).collect()
    }

}
