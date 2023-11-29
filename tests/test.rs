// Import the functions to be tested from the crate root
use console_utils::{input, reveal, select, spinner, SpinnerType};

#[test]
#[ignore]
fn test_input() {
    // Run the function
    let result = input::<u8>("Enter something: ", false);

    // Input anything

    // Check the result
    assert!(result.is_some());
}

#[test]
#[ignore]
fn test_select() {
    // Run the function with simulated input and captured output
    let result = select("Select an option:", &["Option 1"], false, false);

    // select the first option using spacebar and click enter

    // Check the result
    assert!(result.is_some());
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
    reveal("Hello World!\n", 0.1);
}
