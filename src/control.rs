//! Control Utilities
//!
//! This module provides functions for controlling the console, including flushing the output buffer,
//! clearing lines, and moving the cursor in various directions.
use std::io::{self, Write};

/// Flushes the output buffer, ensuring that all content is written to the console.
///
/// # Example
///
/// ```rust
/// use console_utils::control::flush;
///
/// // Flush the output buffer to ensure content is displayed immediately
/// flush();
/// ```
pub fn flush() {
    io::stdout().flush().unwrap();
}

/// Clears the current line in the console.
///
/// This function uses ANSI escape codes to clear the entire line and move the cursor to the
/// beginning of the line.
///
/// # Example
///
/// ```rust
/// use console_utils::control::clear_line;
///
/// // Clear the current line
/// clear_line();
/// ```
pub fn clear_line() {
    print!("\r\x1b[2K");
    flush();
}

/// Moves the cursor down by the specified number of lines.
///
/// # Arguments
///
/// * `n` - The number of lines to move the cursor down.
///
/// # Example
///
/// ```rust
/// use console_utils::control::move_cursor_down;
///
/// // Move the cursor down by 2 lines
/// move_cursor_down(2);
/// ```
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
///
/// # Example
///
/// ```rust
/// use console_utils::control::move_cursor_up;
///
/// // Move the cursor up by 3 lines
/// move_cursor_up(3);
/// ```
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
///
/// # Example
///
/// ```rust
/// use console_utils::control::move_cursor_left;
///
/// // Move the cursor left by 4 characters
/// move_cursor_left(4);
/// ```
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
///
/// # Example
///
/// ```rust
/// use console_utils::control::move_cursor_right;
///
/// // Move the cursor right by 5 characters
/// move_cursor_right(5);
/// ```
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
///
/// # Example
///
/// ```rust
/// use console_utils::control::move_cursor_to;
///
/// // Move the cursor to column 3, row 5
/// move_cursor_to(3, 5);
/// ```
pub fn move_cursor_to(x: usize, y: usize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
    flush();
}
