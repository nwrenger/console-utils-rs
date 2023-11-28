# Console Utility Library

[![Crate](https://img.shields.io/crates/v/console-utils.svg)](https://crates.io/crates/console-utils)
[![API](https://docs.rs/console-utils/badge.svg)](https://docs.rs/console-utils)

A simple Rust library for console-based user input and option selection.

## Input Function

The `input` function reads user input from the console. It prompts the user with a message, reads a line of input, and returns an `Option<String>`.

### Usage

```rust
use console_utils::input;

fn main() {
    // Prompt the user for input
    let user_input = input("Enter something: ", false, false);

    // Process the user input
    match user_input {
        Some(value) => println!("You entered: {}", value),
        None => println!("Input is empty."),
    }
}
```
## Select Function

The `select` function allows the user to interactively select options from a list. It uses arrow keys or 'w' and 's' keys for navigation, spacebar for selection, and Enter to confirm. It returns an `Option<Vec<bool>>` indicating which options were selected.

### Usage

```rust

use console_utils::select;

fn main() {
    let options = vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
    ];

    // Prompt the user to select options
    let selected_options = select("Select an option:", &options, false, false);

    // Process the selected options
    match selected_options {
        Some(selections) => {
            for (i, selected) in selections.iter().enumerate() {
                println!("Option {} selected: {}", i + 1, selected);
            }
        }
        None => println!("No options selected."),
    }
}
```
For more detailed documentation, please refer to the `Rust doc generated documentation`.
