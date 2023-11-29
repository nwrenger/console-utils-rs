//! A simple library for console-based user input and option selection.

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
/// * `before` - The text to display before prompting for input.
/// * `allow_empty` - If true, allows the input to be empty.
/// * `new_line` - If true, adds a newline character after the prompt.
///
/// # Returns
///
/// Returns an `Option<String>` containing the user's input or `None` if the input is empty and
/// `allow_empty` is `false`.
///
/// # Example
///
/// ```no_run
/// use console_utils::input;
///     
/// let user_input = input("Enter something: ", false, false);
///
/// match user_input {
///     Some(value) => println!("You entered: {}", value),
///     None => println!("Input is empty."),
/// }
/// ```
pub fn input(before: &str, allow_empty: bool, new_line: bool) -> Option<String> {
    loop {
        print!("{before} {}", if new_line { '\n' } else { '\0' });
        io::stdout().flush().unwrap();

        let mut cli = String::new();
        io::stdin().read_line(&mut cli).unwrap();

        if allow_empty && cli.trim().is_empty() {
            return None;
        } else if !cli.trim().is_empty() {
            return Some(cli.trim().to_owned());
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
/// selected. Returns `None` if no option was selected and `allow_empty` is `false`.
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
        let stdout = Term::buffered_stdout();

        println!("{}\n", before,);

        for i in options {
            println!("[ ] {}", i);
            matrix.push(false);
        }

        stdout.move_cursor_up(options.len()).unwrap();
        stdout.flush().unwrap();
        let mut i = 0;

        loop {
            if let Ok(character) = stdout.read_key() {
                match character {
                    Key::ArrowUp | Key::Char('w') => {
                        if i > 0 {
                            stdout.move_cursor_up(1).unwrap();
                            i -= 1;
                        }
                    }
                    Key::ArrowDown | Key::Char('s') => {
                        if i < options.len() - 1 {
                            stdout.move_cursor_down(1).unwrap();
                            i += 1;
                        }
                    }
                    Key::Char(' ') => {
                        stdout.clear_line().unwrap();
                        if matrix[i] {
                            stdout.write_line(&format!("[ ] {}", options[i])).unwrap();
                            matrix[i] = false;
                        } else {
                            stdout
                                .write_line(&format!("[{}] {}", style("*").cyan(), options[i]))
                                .unwrap();
                            matrix[i] = true;
                        }
                        stdout.move_cursor_up(1).unwrap();
                        stdout.flush().unwrap();
                    }
                    Key::Enter => {
                        break;
                    }
                    _ => {}
                }
            }
            stdout.flush().unwrap();
        }

        if matrix.iter().filter(|&&selected| selected).count() > 1 && !multiple {
            reset(stdout, "\nPlease Select only one!\n", options.len());
        } else if allow_empty && matrix.iter().all(|&x| !x) {
            reset(stdout, "", options.len());
            return None;
        } else if !matrix.iter().all(|&x| !x) {
            reset(stdout, "", options.len());
            return Some(matrix);
        } else {
            reset(stdout, "\nWrong Input\n", options.len());
        }
    }
}

// Internal function for resetting the console.
fn reset(stdout: Term, mes: &str, len: usize) {
    stdout.move_cursor_down(len).unwrap();
    stdout.flush().unwrap();
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
    let stdout = Term::buffered_stdout();
    let frames = spinner_type.to_frames();
    let mut i = 0;

    while time > 0.0 {
        stdout.clear_line().unwrap();
        stdout.write_line(frames[i]).unwrap();
        stdout.move_cursor_up(1).unwrap();
        stdout.move_cursor_right(frames[i].len()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_secs_f64(0.075));
        time -= 0.075;
        if i < frames.len() - 1 {
            i += 1
        } else {
            i = 0
        }
    }

    stdout.clear_line().unwrap();
    stdout.flush().unwrap();
}
