// Import the functions to be tested from the crate root
use console_utils::{input, select, spinner, SpinnerType};

#[test]
#[ignore]
fn test_input() {
    // Run the function
    let result = input("Enter something: ", false, false);

    // Put in Hello

    // Check the result
    assert_eq!(result, Some("Hello".to_owned()));
}

#[test]
#[ignore]
fn test_select() {
    // Run the function with simulated input and captured output
    let result = select("Select an option:", &["Option 1"], false, false);

    // select the first option using spacebar and click enter

    // Check the result
    assert_eq!(result, Some(vec![true]));
}

#[test]
#[ignore]
fn test_spinner() {
    // Give the fn the needed time and SpinnerType
    spinner(4.2, SpinnerType::Standard);

    // Custom Spinner
    spinner(4.2, SpinnerType::Custom(vec!["1", "2", "3", "4", "3", "2"]))
}
