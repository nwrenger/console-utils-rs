//! A Cross Platform Read Implementation
//!
//! This module provides cross-platform functionality for reading keyboard input,
//! allowing your console application to handle various key events uniformly.

use std::io;

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
    Unknown,
}

/// # Read Key Function
///
/// The `read_key` function reads a single key event from the console input
/// and returns a `Key` enum.
pub fn read_key() -> io::Result<Key> {
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
/// keyboard input. It utilizes the `windows-sys` crate to interact with Windows Console API.
#[cfg(windows)]
pub mod windows {
    use super::Key;
    use std::io;
    use std::mem;
    use windows_sys::Win32::System::Console::{
        GetStdHandle, ReadConsoleInputW, INPUT_RECORD, KEY_EVENT, KEY_EVENT_RECORD,
        STD_INPUT_HANDLE,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse;

    pub(crate) fn read_key() -> io::Result<Key> {
        let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        let mut buffer: INPUT_RECORD = unsafe { mem::zeroed() };

        let mut events_read: u32 = unsafe { mem::zeroed() };

        loop {
            let success = unsafe { ReadConsoleInputW(handle, &mut buffer, 1, &mut events_read) };
            if success == 0 {
                return Err(io::Error::last_os_error());
            }
            if events_read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "ReadConsoleInput returned no events, instead of waiting for an event",
                ));
            }

            if events_read == 1 && buffer.EventType == KEY_EVENT as u16 {
                let key_event: KEY_EVENT_RECORD = unsafe { mem::transmute(buffer.Event) };

                if key_event.bKeyDown != 0 {
                    return match key_event.wVirtualKeyCode {
                        KeyboardAndMouse::VK_UP => Ok(Key::ArrowUp),
                        KeyboardAndMouse::VK_DOWN => Ok(Key::ArrowDown),
                        KeyboardAndMouse::VK_RIGHT => Ok(Key::ArrowRight),
                        KeyboardAndMouse::VK_LEFT => Ok(Key::ArrowLeft),
                        KeyboardAndMouse::VK_RETURN => Ok(Key::Enter),
                        KeyboardAndMouse::VK_TAB => Ok(Key::Tab),
                        KeyboardAndMouse::VK_BACK => Ok(Key::Backspace),
                        KeyboardAndMouse::VK_ESCAPE => Ok(Key::Escape),
                        c => Ok(Key::Char(char::from_u32(c as u32).unwrap_or_default())),
                    };
                }
            }
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
    pub(crate) fn read_key() -> io::Result<Key> {
        let mut buffer = [0; 3];
        disable_line_buffering()?;
        if std::io::stdin().read(&mut buffer).is_ok() {
            enable_line_buffering()?;
            match buffer[0] {
                27 => {
                    // Arrow key sequence
                    if buffer[1] == 91 {
                        match buffer[2] {
                            65 => Ok(Key::ArrowUp),
                            66 => Ok(Key::ArrowDown),
                            67 => Ok(Key::ArrowRight),
                            68 => Ok(Key::ArrowLeft),
                            _ => Ok(Key::Unknown),
                        }
                    } else {
                        Ok(Key::Unknown)
                    }
                }
                b'\n' => Ok(Key::Enter),
                b'\t' => Ok(Key::Tab),
                127 => Ok(Key::Backspace),
                c => Ok(Key::Char(c as char)),
            }
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
