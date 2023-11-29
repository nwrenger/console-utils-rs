//! A simple library for console-based user input, option selection and more.

use console::{style, Key, Term};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

/// Reads user input from the console.
///
/// This function prompts the user with a message (`before`) and reads a line of input from the
/// console. The input can be empty unless `allow_empty` is set to `false`. If `new_line` is set
/// to `true`, a newline character will be printed after the prompt.
///
/// # Arguments
///
/// * `before` - The text to display before prompting for input. Add here `\n` for a new line.
/// * `allow_empty` - If true, allows the input to be empty.
///
/// # Returns
///
/// Returns an `Option<T>` containing the user's input converted to the specified type,
/// or `None` if the input is empty and `allow_empty` is `true`.
///
/// # Example
///
/// ```no_run
/// use console_utils::input;
///     
/// let user_input = input::<String>("Enter something: ", false);
///
/// match user_input {
///     Some(value) => println!("You entered: {}", value),
///     None => panic!("The Input cannot be None, allow_empty is false."),
/// }
/// ```
pub fn input<T>(before: &str, allow_empty: bool) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    loop {
        print!("{before}");
        flush();

        let mut cli = String::new();
        io::stdin().read_line(&mut cli).unwrap();

        if allow_empty && cli.trim().is_empty() {
            return None;
        } else if !cli.trim().is_empty() {
            match cli.trim().parse() {
                Ok(value) => return Some(value),
                Err(_) => println!("\nWrong Input Type\n"),
            }
        } else {
            println!("\nWrong Input\n");
        }
    }
}

/// Allows the user to select options from a list using the console.
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
/// * `allow_empty` - If true, allows the user to exit without selecting any option.
/// * `multiple` - If true, allows the user to select multiple options.
///
/// # Returns
///
/// Returns an `Option<Vec<bool>>` containing a vector of booleans indicating which options were
/// selected. Returns `None` if no option was selected and `allow_empty` is `true`.
///
/// # Example
///
/// ```no_run
/// use console_utils::select;
///
/// let options = vec![
///     "Option 1",
///     "Option 2",
///     "Option 3",
/// ];
///
/// let selected_indices = select("Select an option:", &options, false, false);
///
/// match selected_indices {
///     Some(indices) => {
///         println!("Selected indices: {:?}", indices);
///     }
///     None => {
///         println!("No option selected.");
///     }
/// }
/// ```
pub fn select(
    before: &str,
    options: &[&str],
    allow_empty: bool,
    multiple: bool,
) -> Option<Vec<bool>> {
    loop {
        let mut matrix: Vec<bool> = vec![];
        let mut i = 0;
        let stdout = Term::buffered_stdout();

        // print everything
        println!("{}", before,);

        for i in options {
            println!("[ ] {}", i);
            matrix.push(false);
        }

        // move the cursor to the first item
        move_cursor_up(options.len());
        move_cursor_right(options[i].len() + 4);

        // input reaction loop
        loop {
            if let Ok(character) = stdout.read_key() {
                match character {
                    Key::ArrowUp | Key::Char('w') => {
                        if i > 0 {
                            move_cursor_up(1);
                            move_cursor_left(options[i].len() + 4);
                            i -= 1;
                            move_cursor_right(options[i].len() + 4);
                        }
                    }
                    Key::ArrowDown | Key::Char('s') => {
                        if i < options.len() - 1 {
                            move_cursor_down(1);
                            move_cursor_left(options[i].len() + 4);
                            i += 1;
                            move_cursor_right(options[i].len() + 4);
                        }
                    }
                    Key::Char(' ') => {
                        clear_line();
                        if matrix[i] {
                            print!("[ ] {}", options[i]);
                            matrix[i] = false;
                        } else {
                            print!("[{}] {}", style("*").cyan(), options[i]);
                            matrix[i] = true;
                        }
                        flush();
                    }
                    Key::Enter => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        // process input
        if matrix.iter().filter(|&&selected| selected).count() > 1 && !multiple {
            reset("\nPlease Select only one!\n", options.len());
        } else if allow_empty && matrix.iter().all(|&x| !x) {
            reset("", options.len());
            return None;
        } else if !matrix.iter().all(|&x| !x) {
            reset("", options.len());
            return Some(matrix);
        } else {
            reset("\nPlease Select any option!\n", options.len());
        }
    }
}

// Internal function for resetting the console.
fn reset(mes: &str, len: usize) {
    move_cursor_down(len);
    println!("{mes}");
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
///
/// # Example
///
/// ```rust
/// use console_utils::{spinner, SpinnerType};
///
/// // Display a standard spinner for 3 seconds
/// spinner(3.0, SpinnerType::Standard);
///
/// // Display a custom spinner for 2 seconds
/// spinner(2.0, SpinnerType::Custom(vec!["1", "2", "3", "4", "3", "2"]));
/// ```
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

/// Flushes the output buffer, ensuring that all content is written to the console.
///
/// # Example
///
/// ```rust
/// use console_utils::flush;
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
/// use console_utils::clear_line;
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
/// use console_utils::move_cursor_down;
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
/// use console_utils::move_cursor_up;
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
/// use console_utils::move_cursor_left;
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
/// use console_utils::move_cursor_right;
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
/// use console_utils::move_cursor_to;
///
/// // Move the cursor to column 3, row 5
/// move_cursor_to(3, 5);
/// ```
pub fn move_cursor_to(x: usize, y: usize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
    flush();
}

/// Reveals a string gradually, printing one character at a time with a specified time interval.
///
/// This function is useful for creating a typing effect or slowly displaying information to the user.
///
/// # Arguments
///
/// * `str` - The string to reveal gradually. Add here `\n` for a new line.
/// * `time_between` - The time interval (in seconds) between each revealed character.
///
/// # Example
///
/// ```rust
/// use console_utils::reveal;
///
/// // Display "Hello World!" with a time interval of 0.1 seconds between each character and a new line after it's finished.
/// reveal("Hello World!", 0.1);
/// ```
pub fn reveal(str: &str, time_between: f64) {
    for i in 0..str.len() {
        print!("{}", str.chars().nth(i).unwrap_or(' '));
        flush();
        thread::sleep(Duration::from_secs_f64(time_between));
    }
}
