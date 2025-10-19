//! Input Utilities
//!
//! This module provides functions for handling user input in console applications, including reading user input,
//! selecting options from a list, displaying spinners, and gradually revealing, skippable strings.

use std::{
    io,
    str::FromStr,
    thread,
    time::{Duration, Instant},
};

use crate::{
    control::{clear_line, flush, move_cursor_down, move_cursor_up, Visibility},
    read::{key_pressed_within, read_key, Key},
    styled::{Color, StyledText},
};

/// A Wrapper for allowing empty inputs which then return `None`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Empty<T>(pub Option<T>);

impl<T> FromStr for Empty<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Ok(Empty(None))
        } else {
            s.trim().parse::<T>().map(|v| Empty(Some(v)))
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
        let quest = StyledText::new("?").fg(Color::Red);
        let caret = StyledText::new("›").fg(Color::BrightBlack);
        print!("{quest} {before} {caret} ");
        flush();

        let mut cli = String::new();
        io::stdin().read_line(&mut cli).unwrap();

        match cli.parse() {
            Ok(value) => return value,
            Err(_) => {
                let x = StyledText::new("X").fg(Color::Red);
                println!("\n{x} Invalid Input Type\n")
            }
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
    let quest = StyledText::new("?").fg(Color::Red);
    let caret = StyledText::new("›").fg(Color::BrightBlack);
    println!("{quest} {before} {caret} ");

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
    let quest = StyledText::new("?").fg(Color::Red);
    let caret = StyledText::new("›").fg(Color::BrightBlack);
    println!("{quest} {before} {caret} ");

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

/// Populate function for select/multiselect
fn populate(options: &[&str], matrix: Option<&[bool]>, cursor: usize) {
    for (i, option) in options.iter().enumerate() {
        clear_line();
        if i == cursor {
            let caret = StyledText::new("›").fg(Color::Green);
            let option = if matrix.is_some() && matrix.unwrap()[i] {
                StyledText::new(option).fg(Color::Green)
            } else {
                StyledText::new(option).fg(Color::Cyan)
            };
            println!(" {caret} {option}");
        } else if matrix.is_some() && matrix.unwrap()[i] {
            let option = StyledText::new(option).fg(Color::Green);
            println!("   {}", option);
        } else {
            println!("   {}", option);
        }
    }
    move_cursor_up(options.len());
}

/// Enumeration representing different types of spinners.
#[derive(Debug, Clone)]
pub enum SpinnerType {
    /// Spinner with characters `/` `-` `\` `|`.
    Standard,
    /// Spinner with dots `.` `..` `...` `.....`.
    Dots,
    /// Spinner with box characters `▌` `▀` `▐` `▄`.
    Box,
    /// Spinner with flip characters `_` `_` `_` `-` `\` `'` `´` `-` `_` `_` `_`.
    Flip,
    /// Custom spinner with user-defined frames.
    Custom(&'static [&'static str]),
}

impl SpinnerType {
    /// Returns the frames of the spinner type.
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerType::Standard => &["/", "-", "\\", "|"],
            SpinnerType::Dots => &[".", "..", "...", "....", "...", ".."],
            SpinnerType::Box => &["▌", "▀", "▐", "▄"],
            SpinnerType::Flip => &["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"],
            SpinnerType::Custom(frames) => frames,
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
    let frames = spinner_type.frames();
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

const FAST_GRACE_MS: u64 = 120;
const FAST_GRACE: Duration = Duration::from_millis(FAST_GRACE_MS);

/// Reveals a string gradually, printing one character at a time with a specified time interval.
///
/// Useful for typing effects or slow reveals. Can be sped up with the optional skip key and time.
///
/// # Arguments
///
/// - `str` - The string to reveal gradually. Include `\n` for new lines.
/// - `time_between` - The time interval (in seconds) between each revealed character.
/// - `skip` - If `Some((key, faster_time_between))`, pressing this `key` will temporarily speed up
///   the reveal rate by the `faster_time_between`. The speed-up lasts briefly after the last press (a grace period of 120
///   milliseconds) before returning to the normal pace. Holding or repeatedly pressing the key will
///   extend the fast-forward window. If `None`, the reveal speed cannot be changed.
pub fn reveal(str: &str, time_between: f64, skip: Option<(Key, f64)>) {
    // Sanitize input
    let clamped = if time_between.is_finite() && time_between >= 0.0 {
        time_between
    } else {
        0.0
    };
    let normal_delay = Duration::from_secs_f64(clamped);

    // Precompute fast delay if skipping is enabled
    let (skip_key, fast_delay) = if let Some((key, fast_time_between)) = skip {
        let clamped = if fast_time_between.is_finite() && fast_time_between >= 0.0 {
            fast_time_between
        } else {
            0.0
        };
        let delay = Duration::from_secs_f64(clamped);
        (Some(key), delay)
    } else {
        (None, Duration::from_millis(0))
    };

    // If Some(t), we are in fast mode until `t`
    let mut fast_until: Option<Instant> = None;

    for ch in str.chars() {
        print!("{ch}");
        flush();

        // Decide current delay based on whether fast window is active
        let now = Instant::now();
        let fast_active = fast_until.map_or(false, |t| now < t);
        let delay = if fast_active {
            fast_delay
        } else {
            normal_delay
        };

        // No skip configured → sleep the chosen delay
        let Some(skip_key) = skip_key.clone() else {
            std::thread::sleep(delay);
            continue;
        };

        // Wait up to `delay`, reacting to Tab to (re)enter/extend fast mode
        match key_pressed_within(delay) {
            Ok(Some(k)) if k == skip_key => {
                // Enter/extend fast mode
                fast_until = Some(Instant::now() + FAST_GRACE);

                // Drain immediate Tabs (zero wait) to keep extending the window
                while let Ok(Some(k2)) = key_pressed_within(Duration::from_millis(0)) {
                    if k2 == skip_key {
                        fast_until = Some(Instant::now() + FAST_GRACE);
                    } else {
                        break;
                    }
                }
            }
            _ => {
                // Timer expired; if we were in fast mode and grace elapsed, leave it
                if let Some(t) = fast_until {
                    if Instant::now() >= t {
                        fast_until = None;
                    }
                }
            }
        }
    }
}
