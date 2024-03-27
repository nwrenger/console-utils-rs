use std::{thread, time::Duration};

// Import the functions to be tested from the crate root
use console_utils::{
    control::{clear_line, flush, move_cursor_down, move_cursor_up, Visibility},
    input::{input, multiselect, reveal, select, spinner, Empty, SpinnerType},
    read::{read_key, Key},
};

#[test]
#[ignore = "user inputs"]
fn test_input() {
    // Run the function
    let result = input::<Empty<u8>>("Enter something");

    // Input anything

    // Check the result
    println!("{:?}", result);
}

#[test]
#[ignore = "user inputs"]
fn test_select() {
    // Run the function with simulated input and captured output
    let result = select("Select an option", &["Option 1", "Option 2", "Option 3"]);

    // select the first option using enter

    // Check the result
    println!("{:?}", result);

    // Run the function with simulated input and captured output
    let result = multiselect("Select an option", &["Option 1", "Option 2", "Option 3"]);

    // select the first option using spacebar and click enter

    // Check the result
    println!("{:?}", result);
}

#[test]
#[ignore = "user inputs"]
fn test_read_key() {
    // This test assumes a key press event for the 'a' key
    // You may need to adapt this based on the actual behavior of the platform implementation

    // Read the key
    let key = read_key().unwrap();
    assert_eq!(key, Key::Char('a'));
}

#[test]
fn test_spinner() {
    // Give the fn the needed time and SpinnerType
    spinner(1.0, SpinnerType::Standard);

    // Custom Spinner
    spinner(1.0, SpinnerType::Custom(vec!["1", "2", "3", "4", "3", "2"]))
}

#[test]
fn test_reveal() {
    // Give the fn the str and time.
    reveal("Hello World!", 0.1);
}

#[test]
fn test_clear() {
    // Print Something.
    print!("Hello World");

    // Force update the terminal
    flush();

    // wait
    thread::sleep(Duration::from_secs_f64(1.0));

    // Clear the current line.
    clear_line();
}

#[test]
fn test_curser_visibility() {
    // Print Something.
    println!("Hello World");

    // hide
    Visibility::hide_cursor();
    // wait
    thread::sleep(Duration::from_secs_f64(1.0));

    // on drop the cursor will always be shown again, otherwise use this
    // Visibility::show_cursor();
}

#[test]
fn test_move() {
    // Print Something.
    println!("Hello World");
    println!("Hello World");

    // move
    move_cursor_up(2);

    // wait
    thread::sleep(Duration::from_secs_f64(0.5));

    // move
    move_cursor_down(1);

    // wait
    thread::sleep(Duration::from_secs_f64(0.5));

    // Clear the current line.
    clear_line();
}
