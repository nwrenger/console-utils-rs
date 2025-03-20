# Console Utils

[![crates.io](https://img.shields.io/crates/v/console-utils.svg)](https://crates.io/crates/console-utils)
[![crates.io](https://img.shields.io/crates/d/console-utils.svg)](https://crates.io/crates/console-utils)
[![docs.rs](https://docs.rs/console-utils/badge.svg)](https://docs.rs/console-utils)

_A Rust library for console-based user input, option selection, control, and more._

# Overview

This crate offers utility functions for various console-related operations in Rust programs. From obtaining user input to achieving precise terminal control, its main focus is to remain simple while providing extensive functionality.

## Usage

To use Console Utils in your Rust project, you can add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
console-utils = "1.7.0"
```

After adding the dependency, you can import the modules you need in your Rust code. For example:

```rust
use console_utils::input::{input, select};
use console_utils::control::{flush, clear_line};
```

## Examples

### Reading User Input

```rust, no_run
use console_utils::input::input;
// Read user input as a string
let user_input: String = input("Enter something: ");

println!("You entered: {}", user_input);
```

### Selecting Options

#### Single Option
```rust, no_run
use console_utils::input::select;
let options = [
    "Option 1",
    "Option 2",
    "Option 3",
];
// Allow the user to select one option
let selected_index = select("Select an option:", &options);

println!("Selected option: {}", options[selected_index]);
```

#### Multiple Options
```rust, no_run
use console_utils::input::multiselect;
let options = [
    "Option 1",
    "Option 2",
    "Option 3",
];
// Allow the user to select multiple options
let selected_indices = multiselect("Select options:", &options);

println!("Selected indices: {:?}", selected_indices);
```

### Text styling

```rust
use console_utils::styled::{StyledText, Color};

let text = StyledText::new("Hello, world!")
    .fg(Color::Red)
    .bg(Color::Black)
    .bold()
    .underline();

// Prints now a `Hello, world!` with red, bold and underlined text on a black background
println!("{}", text);
```

### Console Control

```rust
use console_utils::control::{flush, clear_line};
// Flush the output buffer to ensure content is displayed immediately
flush();
// Clear the current line in the console
clear_line();
// and more...
// Consult the docs for more details!
```

### Reading Key

```rust, no_run
use console_utils::read::{read_key};
// Cross-platform key reading
let key = read_key();
println!("Pressed key: {:?}", key);
```

### Displaying a Spinner

```rust
use console_utils::input::{spinner, SpinnerType};
// Display a standard spinner for 3 seconds
spinner(3.0, SpinnerType::Standard);
// Display a custom spinner for 2 seconds
spinner(2.0, SpinnerType::Custom(&["1", "2", "3", "4", "3", "2"]));
```

### Gradual String Reveal

```rust
use console_utils::input::reveal;
// Display "Hello World!" with a time interval of 0.1 seconds between each character
reveal("Hello World!", 0.1);
```

For more detailed documentation, please refer to the [generated Rust Docs](https://docs.rs/console-utils/latest/console_utils/).
