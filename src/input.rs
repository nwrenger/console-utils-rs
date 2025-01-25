//! Input Utilities
//!
//! This module provides functions for handling user input in console applications, including reading user input,
//! selecting options from a list, displaying spinners, and gradually revealing strings.
use std::{io, str::FromStr, thread, time::Duration};

use crate::{
    control::*,
    read::{read_key, Key},
};

/// A Wrapper for empty inputs returning a None
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Default)]
pub enum Empty<T> {
    Some(T),
    #[default]
    None,
}

impl<T> FromStr for Empty<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Ok(Empty::None)
        } else {
            s.trim().parse::<T>().map(Empty::Some)
        }
    }
}

/// Reads user input from the console.
///
/// This function prompts the user with a message (`before`) and reads a line of input from the
/// console. The input can be empty.
///
/// # Arguments
///
/// * `before` - The text to display before prompting for input. Add here `\n` for a new line.
///
/// # Returns
///
/// Returns an `T` containing the user's input converted to the specified type.
pub fn input<T>(before: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    loop {
        print!("\x1b[31m?\x1b[0m {before} \x1b[90m›\x1b[0m ");
        flush();

        let mut cli = String::new();
        io::stdin().read_line(&mut cli).unwrap();

        match cli.parse() {
            Ok(value) => return value,
            Err(_) => println!("\n\x1b[31mX\x1b[0m Invalid Input Type\n"),
        }
    }
}

/// Allows the user to select one option from a list using the console.
///
/// This function displays a list of options. The user can navigate through the
/// options using arrow keys or 'w' and 's' keys. If the user presses Enter, the
/// function returns the selected option.
///
/// # Arguments
///
/// * `before` - The text to display before the list of options.
/// * `options` - A vector of strings representing the available options.
///
/// # Returns
///
/// Returns an `usize` as an index of the inputted array `options`
pub fn select<'a>(before: &'a str, options: &'a [&'a str]) -> usize {
    let mut i = 0;

    // print everything
    println!("\x1b[31m?\x1b[0m {before} \x1b[90m›\x1b[0m ");

    populate(options, None, 0);

    // hide cursor
    let vis = Visibility::new();
    vis.hide_cursor();

    loop {
        if let Ok(character) = read_key() {
            match character {
                Key::ArrowUp | Key::Char('w') | Key::Char('W') => {
                    if i > 0 {
                        i -= 1;
                        populate(options, None, i);
                    }
                }
                Key::ArrowDown | Key::Char('s') | Key::Char('S') => {
                    if i < options.len() - 1 {
                        i += 1;
                        populate(options, None, i);
                    }
                }
                Key::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    // reset cursor
    move_cursor_down(options.len());

    i
}

/// Allows the user to select multiple options from a list using the console.
///
/// This function displays a list of options with checkboxes. The user can navigate through the
/// options using arrow keys or 'w' and 's' keys. Pressing the spacebar toggles the selection of
/// the current option. If the user presses Enter, the function returns a vector of booleans
/// indicating which options were selected.
///
/// # Arguments
///
/// * `before` - The text to display before the list of options.
/// * `options` - A vector of strings representing the available options.
///
/// # Returns
///
/// Returns an `Vec<bool>` containing a vector of booleans indicating which options were
/// selected.
pub fn multiselect(before: &str, options: &[&str]) -> Vec<bool> {
    let mut matrix: Vec<bool> = vec![false; options.len()];
    let mut i = 0;

    // print everything
    println!("\x1b[31m?\x1b[0m {before} \x1b[90m›\x1b[0m ");

    populate(options, Some(&matrix), 0);

    // hide cursor
    let vis = Visibility::new();
    vis.hide_cursor();

    loop {
        if let Ok(character) = read_key() {
            match character {
                Key::ArrowUp | Key::Char('w') | Key::Char('W') => {
                    if i > 0 {
                        i -= 1;
                        populate(options, Some(&matrix), i);
                    }
                }
                Key::ArrowDown | Key::Char('s') | Key::Char('S') => {
                    if i < options.len() - 1 {
                        i += 1;
                        populate(options, Some(&matrix), i);
                    }
                }
                Key::Char(' ') => {
                    move_cursor_down(i);
                    clear_line();
                    matrix[i] = !matrix[i];
                    flush();
                    move_cursor_up(i);
                    populate(options, Some(&matrix), i);
                }
                Key::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    // reset cursor
    move_cursor_down(options.len());

    matrix
}

/// Populate function for multiselect
fn populate(options: &[&str], matrix: Option<&[bool]>, cursor: usize) {
    for (i, option) in options.iter().enumerate() {
        clear_line();
        if i == cursor {
            println!(
                "\x1b[36m ›\x1b[0m\x1b[3{}m {}\x1b[0m",
                if matrix.is_some() && matrix.unwrap()[i] {
                    "2"
                } else {
                    "6"
                },
                option
            );
        } else if matrix.is_some() && matrix.unwrap()[i] {
            println!("\x1b[32m   {}\x1b[0m", option);
        } else {
            println!("   {}", option);
        }
    }
    move_cursor_up(options.len());
}

/// Enumeration representing different types of spinners.
#[derive(Debug, Clone)]
pub enum SpinnerType {
    Standard,
    Dots,
    Box,
    Flip,
    Custom(Vec<&'static str>),
}

impl SpinnerType {
    /// Converts the spinner type to a vector of frames, gives back the following variants:
    ///  - `SpinnerType::Standard`: Standard spinner with characters / - \ |.
    ///  - `SpinnerType::Dots`: Spinner with dots . .. ... .....
    ///  - `SpinnerType::Box`: Spinner with box characters ▌ ▀ ▐ ▄.
    ///  - `SpinnerType::Flip`: Spinner with flip characters _ _ _ - \ ' ´ - _ _ _.
    ///  - `SpinnerType::Custom(frames)`: Custom spinner with user-defined frames.
    pub fn to_frames(&self) -> Vec<&'static str> {
        match self {
            SpinnerType::Standard => vec!["/", "-", "\\", "|"],
            SpinnerType::Dots => vec![".", "..", "...", "....", "...", ".."],
            SpinnerType::Box => vec!["▌", "▀", "▐", "▄"],
            SpinnerType::Flip => vec!["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"],
            SpinnerType::Custom(frames) => frames.to_owned(),
        }
    }
}

/// Displays a console-based spinner animation.
///
/// A spinner is a visual indicator of a long-running process. It consists of a set of frames
/// that are displayed sequentially to create the appearance of motion.
///
/// # Parameters
///
/// - `time`: A floating-point number representing the duration of the spinner animation in seconds.
/// - `spinner_type`: The type of spinner to display.
pub fn spinner(mut time: f64, spinner_type: SpinnerType) {
    let frames = spinner_type.to_frames();
    let mut i = 0;

    while time > 0.0 {
        clear_line();
        print!("{}", frames[i]);
        flush();
        thread::sleep(Duration::from_secs_f64(0.075));
        time -= 0.075;
        if i < frames.len() - 1 {
            i += 1
        } else {
            i = 0
        }
    }

    clear_line();
}

/// Reveals a string gradually, printing one character at a time with a specified time interval.
///
/// This function is useful for creating a typing effect or slowly displaying information to the user.
///
/// # Arguments
///
/// * `str` - The string to reveal gradually. Add here `\n` for a new line.
/// * `time_between` - The time interval (in seconds) between each revealed character.
pub fn reveal(str: &str, time_between: f64) {
    for i in 0..str.len() {
        print!("{}", str.chars().nth(i).unwrap_or(' '));
        flush();
        thread::sleep(Duration::from_secs_f64(time_between));
    }
}
