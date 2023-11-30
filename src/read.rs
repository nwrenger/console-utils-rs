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
pub fn read_key() -> Option<Key> {
    #[cfg(windows)]
    {
        windows::read_key()
    }

    #[cfg(unix)]
    {
        unix::read_key()
    }
}

/// # Windows Module
///
/// The `windows` module contains Windows-specific implementation details for reading
/// keyboard input. It utilizes the `windows` crate to interact with Windows Console API.
#[cfg(windows)]
pub mod windows {
    use std::io::{self, Read};
    use windows_sys::Win32::Foundation::E_HANDLE;
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT,
    };

    use super::Key;

    // Internal function for disabling line buffering.
    fn disable_line_buffering() -> io::Result<()> {
        let handle = unsafe { GetStdHandle(E_HANDLE.try_into().unwrap()) };

        let mut mode: u32 = 0;
        unsafe {
            if GetConsoleMode(handle, &mut mode) == 0 {
                return Err(io::Error::last_os_error());
            }

            if SetConsoleMode(handle, mode & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT)) == 0 {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(())
    }

    // Internal function for enabling line buffering.
    fn enable_line_buffering() -> io::Result<()> {
        let handle = unsafe { GetStdHandle(E_HANDLE.try_into().unwrap()) };

        let mut mode: u32 = 0;
        unsafe {
            if GetConsoleMode(handle, &mut mode) == 0 {
                return Err(io::Error::last_os_error());
            }

            if SetConsoleMode(handle, mode | (ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT)) == 0 {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(())
    }

    // Internal function for reading a key from the console.
    pub(crate) fn read_key() -> Option<Key> {
        let mut buffer = [0; 3];
        disable_line_buffering().unwrap();
        if std::io::stdin().read(&mut buffer).is_ok() {
            enable_line_buffering().unwrap();
            match buffer[0] {
                13 => Some(Key::Enter),
                9 => Some(Key::Tab),
                8 => Some(Key::Backspace),
                27 => Some(Key::Escape),
                38 => Some(Key::ArrowUp),
                40 => Some(Key::ArrowDown),
                39 => Some(Key::ArrowRight),
                37 => Some(Key::ArrowLeft),
                c => Some(Key::Char(c as char)),
            }
        } else {
            None
        }
    }
}

/// # Unix Module
///
/// The `unix` module contains Unix-specific implementation details for reading
/// keyboard input. It uses the `libc` crate to manipulate terminal attributes.
#[cfg(unix)]
pub mod unix {
    use libc::{tcgetattr, tcsetattr, ECHO, ICANON, STDIN_FILENO, TCSANOW};
    use std::io::{self, Read};
    use std::mem;

    use super::Key;

    // Internal function for disabling line buffering.
    fn disable_line_buffering() -> io::Result<()> {
        let mut termios = unsafe { mem::zeroed() };
        if unsafe { tcgetattr(STDIN_FILENO, &mut termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        termios.c_lflag &= !(ICANON | ECHO);

        if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    // Internal function for enabling line buffering.
    fn enable_line_buffering() -> io::Result<()> {
        let mut termios = unsafe { mem::zeroed() };
        if unsafe { tcgetattr(STDIN_FILENO, &mut termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        termios.c_lflag |= ICANON | ECHO;

        if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    // Internal function for reading a key from the console.
    pub(crate) fn read_key() -> Option<Key> {
        let mut buffer = [0; 3];
        disable_line_buffering().unwrap();
        if std::io::stdin().read(&mut buffer).is_ok() {
            enable_line_buffering().unwrap();
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
