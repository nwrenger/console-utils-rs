// Import the functions to be tested from the crate root
use console_utils::{input, select};

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
    let result = select("Select an option:", &["Option 1".to_string()], false, false);

    // select the first option using spacebar and click enter

    // Check the result
    assert_eq!(result, Some(vec![true]));
}
