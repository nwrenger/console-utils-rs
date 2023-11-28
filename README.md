# Console Utility Library

[![Crate](https://img.shields.io/crates/v/console-utils.svg)](https://crates.io/crates/console-utils)
[![API](https://docs.rs/console-utils/badge.svg)](https://docs.rs/console-utils)

A simple Rust library for console-based user input, option selection and more.

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
        "Option 1",
        "Option 2",
        "Option 3",
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

## Spinner Function

The spinner function creates a console-based spinner animation, offering a visually appealing way to indicate ongoing processes.

### Usage

```rust
use console_utils::{spinner, SpinnerType};

// Display a standard spinner for 3 seconds
spinner(3.0, SpinnerType::Standard);

// Display a dots spinner for 2 seconds
spinner(2.0, SpinnerType::Dots);

// Display a custom spinner for 1 second (using a custom frame)
spinner(1.0, SpinnerType::Custom("@ "));

// Display a box spinner for 1.5 seconds
spinner(1.5, SpinnerType::Box);

// Display a flip spinner for 2 seconds
spinner(2.0, SpinnerType::Flip);
```

For more detailed documentation, please refer to the [generated Rust Docs](https://crates.io/crates/console-utils).
