//! Read Implementation Cross Platform
//!
//! This module provides cross-platform functionality for reading keyboard input,
//! allowing your console application to handle various key events uniformly.
//! # Examples
//!
//! ```no_run
//! use console_utils::read::{Key, read_key};
//!
//! // Cross-platform key reading example
//! let key = read_key().unwrap();
//!
//! println!("Pressed key: {:?}", key);
//! ```

/// # Key Enum
///
/// The `Key` enum represents different keyboard keys that can be captured by the
/// `read_key` function.
///
/// - `ArrowUp`: Represents the arrow up key.
/// - `ArrowDown`: Represents the arrow down key.
/// - `ArrowRight`: Represents the arrow right key.
/// - `ArrowLeft`: Represents the arrow left key.
/// - `Enter`: Represents the Enter/Return key.
/// - `Tab`: Represents the Tab key.
/// - `Backspace`: Represents the Backspace key.
/// - `Escape`: Represents the Escape key.
/// - `Char(char)`: Represents any printable character on the keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    Enter,
    Tab,
    Backspace,
    Escape,
    Char(char),
}

/// # Read Key Function
///
/// The `read_key` function reads a single key event from the console input
/// and returns a `Key` enum.
pub fn read_key() -> Result<Key, std::io::Error> {
    #[cfg(target_os = "windows")]
    {
        windows::read_key()
    }

    #[cfg(not(target_os = "windows"))]
    {
        unix::read_key().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to read a single key")
        })
    }
}

/// # Windows Module
///
/// The `windows` module contains Windows-specific implementation details for reading
/// keyboard input. It utilizes the `winapi` crate to interact with Windows Console API.
#[cfg(target_os = "windows")]
pub mod windows {
    use std::io;
    use winapi::{
        shared::minwindef::{DWORD, FALSE, TRUE},
        um::consoleapi::{GetNumberOfConsoleInputEvents, ReadConsoleInputW, KEY_EVENT},
        um::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT},
        um::winnt::HANDLE,
        um::winuser::{GetConsoleMode, GetStdHandle, SetConsoleMode, STD_INPUT_HANDLE},
    };

    use super::Key;

    // Windows specific constants
    const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;

    fn key_from_key_code(key_code: DWORD) -> Key {
        match key_code {
            13 => Key::Enter,
            9 => Key::Tab,
            8 => Key::Backspace,
            27 => Key::Escape,
            38 => Key::ArrowUp,
            40 => Key::ArrowDown,
            39 => Key::ArrowRight,
            37 => Key::ArrowLeft,
            _ => Key::Char(key_code as u8 as char),
        }
    }

    fn setup_console_mode(handle: HANDLE) -> io::Result<(DWORD, DWORD)> {
        let mut mode: DWORD = 0;
        if unsafe { GetConsoleMode(handle, &mut mode) } == FALSE {
            return Err(io::Error::last_os_error());
        }

        let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_INPUT;
        if unsafe { SetConsoleMode(handle, new_mode) } == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok((mode, new_mode))
    }

    fn restore_console_mode(handle: HANDLE, original_mode: DWORD) -> io::Result<()> {
        if unsafe { SetConsoleMode(handle, original_mode) } == FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub(crate) fn read_key() -> io::Result<Key> {
        // Get the console handle for standard input
        let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        if handle == HANDLE::NULL {
            return Err(io::Error::last_os_error());
        }

        // Save the original console mode and set the new mode with virtual terminal input enabled
        let (original_mode, new_mode) = setup_console_mode(handle)?;

        // Read console input events
        let mut events: Vec<KEY_EVENT> = Vec::with_capacity(1);
        let mut num_events: DWORD = 0;

        if unsafe { ReadConsoleInputW(handle, events.as_mut_ptr() as *mut _, 1, &mut num_events) }
            == FALSE
        {
            let error = io::Error::last_os_error();
            // Restore the original console mode before returning the error
            restore_console_mode(handle, original_mode)?;
            return Err(error);
        }

        // Restore the original console mode
        restore_console_mode(handle, original_mode)?;

        if num_events > 0 {
            let event = &events[0];
            if event.bKeyDown == TRUE {
                return Ok(key_from_key_code(event.wVirtualKeyCode));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to read a single key",
        ))
    }
}

/// # Unix Module
///
/// The `unix` module contains Unix-specific implementation details for reading
/// keyboard input. It uses the `termios` crate to manipulate terminal attributes.
#[cfg(not(target_os = "windows"))]
pub mod unix {
    use std::io::Read;

    use termios::{tcsetattr, Termios, TCSANOW};

    use super::Key;

    // Internal function for disabling line buffering.
    fn disable_line_buffering() {
        let mut termios = Termios::from_fd(0).expect("Failed to get terminal attributes");
        termios.c_lflag &= !(termios::ICANON | termios::ECHO);
        tcsetattr(0, TCSANOW, &termios).expect("Failed to set terminal attributes");
    }

    // Internal function for enabling line buffering.
    fn enable_line_buffering() {
        let mut termios = Termios::from_fd(0).expect("Failed to get terminal attributes");
        termios.c_lflag |= termios::ICANON | termios::ECHO;
        tcsetattr(0, TCSANOW, &termios).expect("Failed to set terminal attributes");
    }

    // Internal function for reading a key from the console.
    pub(crate) fn read_key() -> Option<Key> {
        let mut buffer = [0; 3];
        disable_line_buffering();
        if std::io::stdin().read(&mut buffer).is_ok() {
            enable_line_buffering();
            match buffer[0] {
                27 => {
                    // Arrow key sequence
                    if buffer[1] == 91 {
                        match buffer[2] {
                            65 => Some(Key::ArrowUp),
                            66 => Some(Key::ArrowDown),
                            67 => Some(Key::ArrowRight),
                            68 => Some(Key::ArrowLeft),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                b'\n' => Some(Key::Enter),
                b'\t' => Some(Key::Tab),
                127 => Some(Key::Backspace),
                c => Some(Key::Char(c as char)),
            }
        } else {
            None
        }
    }
}
