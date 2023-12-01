# Console Utils

[![Crate](https://img.shields.io/crates/v/console-utils.svg)](https://crates.io/crates/console-utils)
[![API](https://docs.rs/console-utils/badge.svg)](https://docs.rs/console-utils)

_A Rust library for console-based user input, option selection, control, and more._

# Overview

This crate offers utility functions for various console-related operations in Rust programs. From obtaining user input to achieving precise terminal control, its main focus is to remain simple while providing extensive functionality.



## Usage

To use Console Utils in your Rust project, you can add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
console-utils = "1.5.6"
```

After adding the dependency, you can import the modules you need in your Rust code. For example:

```rust
use console_utils::input::{input, select};
use console_utils::control::{flush, clear_line};
```

## Example

```rust
use console_utils::{input::{input, spinner, SpinnerType, select, reveal}, control::clear_line};

fn main() {
    // Read user input as a string
    let user_input: Option<String> = input("Enter something: ", false);
    
    match user_input {
        Some(value) => println!("You entered: {}", value),
        None => panic!("Input cannot be None when 'allow_empty' is set to false."),
    }

    // Display a standard spinner for 3 seconds
    spinner(3.0, SpinnerType::Standard);

    // Cross-platform key reading
    let key = read_key();

    println!("Pressed key: {:?}", key);

    let options = vec![
        "Option 1",
        "Option 2",
        "Option 3",
    ];

    // Allow the user to select one option
    let selected_indices = select("Select an option:", &options, false, false);

    match selected_indices {
        Some(indices) => println!("Selected indices: {:?}", indices),
        None => panic!("The Options cannot be None, allow_empty is false."),
    }
    
    // Display "Hello World!" with a time interval of 0.1 seconds between each character
    reveal("Hello World!", 0.1);

    // Clear the current line in the console, so the "Hello World!"
    clear_line();
}
```

For more detailed documentation, please refer to the [generated Rust Docs](https://docs.rs/console-utils/latest/console_utils/).
