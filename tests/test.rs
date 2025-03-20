use std::{thread, time::Duration};

// Import the functions to be tested from the crate root
use console_utils::{
    control::{clear_line, flush, move_cursor_down, move_cursor_up, Visibility},
    input::{input, multiselect, reveal, select, spinner, Empty, SpinnerType},
    read::{read_key, Key},
    styled::{Color, StyledText},
};

#[test]
#[ignore = "user inputs"]
fn user_input() {
    // Run the function
    let result = input::<Empty<u8>>("Enter something (integer)");

    // Input anything

    // Check the result
    println!("{:?}", result);
}

#[test]
#[ignore = "user inputs"]
fn user_select() {
    // Run the function with simulated input and captured output
    let result = select(
        "Select the one option (select using Enter)",
        &["Option 1", "Option 2", "Option 3"],
    );

    // select the first option using enter

    // Check the result
    println!("{:?}", result);

    // Run the function with simulated input and captured output
    let result = multiselect(
        "Select an option (select using SpaceBar, then Enter)",
        &["Option 1", "Option 2", "Option 3"],
    );

    // select the first option using spacebar and click enter

    // Check the result
    println!("{:?}", result);
}

#[test]
#[ignore = "user inputs"]
fn user_read_key() {
    // This test assumes a key press event for the 'a' key
    // You may need to adapt this based on the actual behavior of the platform implementation
    println!("Input 'a' key");

    // Read the key
    let key = read_key().unwrap();
    assert_eq!(key, Key::Char('a'));
}

#[test]
fn spinner_visible() {
    // Give the fn the needed time and SpinnerType
    spinner(1.0, SpinnerType::Standard);

    // Custom Spinner
    spinner(1.0, SpinnerType::Custom(&["1", "2", "3", "4", "3", "2"]))
}

#[test]
fn reveal_visible() {
    // Give the fn the str and time.
    reveal("Hello World!", 0.1);
}

#[test]
fn clear() {
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
fn cursor_visibility() {
    // Print Something.
    println!("Hello World");

    let vis = Visibility::new();

    // hide
    vis.hide_cursor();
    // wait
    thread::sleep(Duration::from_secs_f64(1.0));

    // on drop the cursor will always be shown again, otherwise use this
    // vis.show_cursor();
}

#[test]
fn r#move() {
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

#[test]
fn color() {
    // fg
    println!("{}", StyledText::new("This is red").fg(Color::BrightRed));

    // bg
    println!(
        "{}",
        StyledText::new("This has red bg").bg(Color::BrightRed)
    );

    // all variants
    println!("{}", StyledText::new("This is bold").bold());
    println!("{}", StyledText::new("This is italic").italic());
    println!("{}", StyledText::new("This is underline").underline());
    println!("{}", StyledText::new("This blinks").blink());
    println!("{}", StyledText::new("This has reversed colors").reverse());
    println!(
        "{}",
        StyledText::new("This is strikethrough").strikethrough()
    );

    // some combined
    println!(
        "{}",
        StyledText::new("This is special")
            .fg(Color::BrightCyan)
            .bg(Color::Yellow)
            .bold()
            .blink()
    );
}
