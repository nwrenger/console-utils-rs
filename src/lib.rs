//! Console Utils - A Rust library for console-based user input, option selection, control, and more.
//!
//! This crate offers utility functions for various console-related operations in Rust programs. From obtaining user input to achieving precise terminal control, its main focus is to remain simple while providing extensive functionality.
//!
//! # Getting Started
//!
//! To use Console Utils in your Rust project, you can add the following dependency to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! console-utils = "1.5.4"
//! ```
//!
//! After adding the dependency, you can import the modules you need in your Rust code. For example:
//!
//! ```rust
//! use console_utils::input::{input, select};
//! use console_utils::control::{flush, clear_line};
//! ```
//!
//! # Examples
//!
//! ## Reading User Input
//!
//! ```no_run
//! use console_utils::input::input;
//!
//! // Read user input as a string
//! let user_input: Option<String> = input("Enter something: ", false);
//!
//! match user_input {
//!     Some(value) => println!("You entered: {}", value),
//!     None => panic!("Input cannot be None when 'allow_empty' is set to false."),
//! }
//! ```
//!
//! ## Selecting Options
//!
//! ```no_run
//! use console_utils::input::select;
//!
//! let options = vec![
//!     "Option 1",
//!     "Option 2",
//!     "Option 3",
//! ];
//!
//! // Allow the user to select one option
//! let selected_indices = select("Select an option:", &options, false, false);
//!
//! match selected_indices {
//!     Some(indices) => println!("Selected indices: {:?}", indices),
//!     None => panic!("The Options cannot be None, allow_empty is false."),
//! }
//! ```
//!
//! ## Console Control
//!
//! ```rust
//! use console_utils::control::{flush, clear_line};
//!
//! // Flush the output buffer to ensure content is displayed immediately
//! flush();
//!
//! // Clear the current line in the console
//! clear_line();
//! ```
//!
//! ## Displaying a Spinner
//!
//! ```rust
//! use console_utils::input::{spinner, SpinnerType};
//!
//! // Display a standard spinner for 3 seconds
//! spinner(3.0, SpinnerType::Standard);
//!
//! // Display a custom spinner for 2 seconds
//! spinner(2.0, SpinnerType::Custom(vec!["1", "2", "3", "4", "3", "2"]));
//! ```
//!
//! ## Gradual String Reveal
//!
//! ```rust
//! use console_utils::input::reveal;
//!
//! // Display "Hello World!" with a time interval of 0.1 seconds between each character
//! reveal("Hello World!", 0.1);
//! ```

pub mod control;
pub mod input;
pub mod read;
