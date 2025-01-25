//! Control Utilities
//!
//! This module provides functions for controlling the console, including flushing the output buffer,
//! clearing lines, and moving the cursor in various directions.
use std::io::{self, Write};

/// Flushes the output buffer, ensuring that all content is written to the console.
pub fn flush() {
    io::stdout().flush().unwrap();
}

/// Clears the current line in the console.
///
/// This function uses ANSI escape codes to clear the entire line and move the cursor to the
/// beginning of the line.
pub fn clear_line() {
    print!("\r\x1b[2K");
    flush();
}

/// Clears the `i` lines in the console.
pub fn clear_lines(i: usize) {
    for _ in 0..i {
        print!("\r\x1b[2K");
        flush();
    }
}

/// Struct for ensuring and changing cursor visibility.
#[derive(Default)]
pub struct Visibility;

impl Visibility {
    pub fn new() -> Self {
        Self
    }

    /// Hide the cursor via an ASCII escape sequence.
    pub fn hide_cursor(&self) {
        print!("\x1B[?25l");
        flush();
    }

    /// Show the cursor via an ASCII escape sequence.
    pub fn show_cursor(&self) {
        print!("\x1B[?25h");
        flush();
    }
}

impl Drop for Visibility {
    fn drop(&mut self) {
        Visibility::show_cursor(self);
    }
}

/// Moves the cursor down by the specified number of lines.
///
/// # Arguments
///
/// * `n` - The number of lines to move the cursor down.
pub fn move_cursor_down(n: usize) {
    if n > 0 {
        print!("\x1b[{}B", n);
        flush();
    }
}

/// Moves the cursor up by the specified number of lines.
///
/// # Arguments
///
/// * `n` - The number of lines to move the cursor up.
pub fn move_cursor_up(n: usize) {
    if n > 0 {
        print!("\x1b[{}A", n);
        flush();
    }
}

/// Moves the cursor to the left by the specified number of characters.
///
/// # Arguments
///
/// * `n` - The number of characters to move the cursor to the left.
pub fn move_cursor_left(n: usize) {
    if n > 0 {
        print!("\x1b[{}D", n);
        flush();
    }
}

/// Moves the cursor to the right by the specified number of characters.
///
/// # Arguments
///
/// * `n` - The number of characters to move the cursor to the right.
pub fn move_cursor_right(n: usize) {
    if n > 0 {
        print!("\x1b[{}C", n);
        flush();
    }
}

/// Moves the cursor to the specified position on the console.
///
/// # Arguments
///
/// * `x` - The horizontal position (column) to move the cursor to.
/// * `y` - The vertical position (row) to move the cursor to.
pub fn move_cursor_to(x: usize, y: usize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
    flush();
}
