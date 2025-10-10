//! A Cross Platform Read Implementation
//!
//! This module provides functions for reading keys and waiting for key presses until a specified timeout,
//! allowing your console application to handle keyboard events consistently across platforms.

use std::{io, time::Duration};

/// Represents different keyboard keys that can be captured by the `read_key` function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// Arrow up key.
    ArrowUp,
    /// Arrow down key.
    ArrowDown,
    /// Arrow right key.
    ArrowRight,
    /// Arrow left key.
    ArrowLeft,
    /// Enter/Return key.
    Enter,
    /// Tab key.
    Tab,
    /// Backspace key.
    Backspace,
    /// Escape key.
    Escape,
    /// Any printable character on the keyboard.
    Char(char),
    /// Any unrecognized key.
    Unknown,
}

/// Reads a single key event from the console input and returns a `Key` enum.
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

/// Waits for a key press for up to the specified `timeout` duration.
pub fn key_pressed_within(timeout: Duration) -> io::Result<Option<Key>> {
    #[cfg(windows)]
    {
        windows::key_pressed_within(timeout)
    }
    #[cfg(unix)]
    {
        unix::key_pressed_within(timeout)
    }
}

/// Contains Windows-specific implementation details for reading keyboard
/// input. It utilizes the `windows-sys` crate to interact with Windows Console API.
#[cfg(windows)]
pub mod windows {
    use super::Key;
    use std::io;
    use std::mem;
    use std::os::windows::raw::HANDLE;
    use std::time::Instant;
    use windows_sys::Win32::Foundation::{INVALID_HANDLE_VALUE, WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows_sys::Win32::System::Console::{
        GetStdHandle, PeekConsoleInputW, ReadConsoleInputW, INPUT_RECORD, KEY_EVENT,
        KEY_EVENT_RECORD, STD_INPUT_HANDLE,
    };
    use windows_sys::Win32::System::Threading::WaitForSingleObject;
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

    pub(super) fn key_pressed_within(timeout: std::time::Duration) -> std::io::Result<Option<Key>> {
        // Peek the next record; if it's noise (non-key or key-up), consume it and return Ok(false).
        // If a key-down is pending, leave it in the buffer and return Ok(true) so read_key() can take it.
        unsafe fn ensure_head_is_keydown_or_empty(handle: HANDLE) -> io::Result<bool> {
            let mut rec: INPUT_RECORD = mem::zeroed();
            let mut read: u32 = 0;

            // Peek exactly one record; if none, buffer is empty.
            if PeekConsoleInputW(handle, &mut rec, 1, &mut read) == 0 {
                return Err(io::Error::last_os_error());
            }
            if read == 0 {
                return Ok(false); // empty buffer
            }

            if rec.EventType == KEY_EVENT as u16 {
                // SAFETY: union access matches Win32 layout.
                let key: KEY_EVENT_RECORD = mem::transmute(rec.Event);
                if key.bKeyDown != 0 {
                    // key-down at head; leave it for read_key()
                    return Ok(true);
                }
                // key-up at head: consume it and report "no key-down yet"
                let mut took: u32 = 0;
                if ReadConsoleInputW(handle, &mut rec, 1, &mut took) == 0 {
                    return Err(io::Error::last_os_error());
                }
                return Ok(false);
            }

            // Non-key at head: consume it and report "no key-down yet"
            let mut took: u32 = 0;
            if ReadConsoleInputW(handle, &mut rec, 1, &mut took) == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(false)
        }

        let handle: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        if handle == 0 as HANDLE || handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        let deadline = Instant::now() + timeout;

        loop {
            // Clean the head of the queue: remove non-keys and key-ups.
            // If a key-down is already pending, read it now.
            if unsafe { ensure_head_is_keydown_or_empty(handle)? } {
                return Ok(Some(read_key()?));
            }

            // Remaining time?
            let now = Instant::now();
            if now >= deadline {
                return Ok(None);
            }
            let remaining_ms = ((deadline - now).as_millis()).min(u32::MAX as u128) as u32;

            // Wait for *any* console input; when signaled, loop to filter again.
            match unsafe { WaitForSingleObject(handle, remaining_ms) } {
                WAIT_TIMEOUT => return Ok(None),
                WAIT_OBJECT_0 => continue,
                _ => return Err(io::Error::last_os_error()),
            }
        }
    }
}

/// Contains Unix-specific implementation details for reading keyboard
/// input. It uses the `libc` crate to manipulate terminal attributes.
#[cfg(unix)]
pub mod unix {
    use libc::{
        poll, pollfd, tcgetattr, tcsetattr, termios, ECHO, ICANON, POLLIN, STDIN_FILENO, TCSANOW,
    };
    use std::io::{self, Read};
    use std::mem;
    use std::time::Duration;

    use super::Key;

    /// Small helper: fetch current termios for a given fd.
    fn get_termios(fd: i32) -> io::Result<termios> {
        // SAFETY: zeroed termios is immediately filled by tcgetattr on success.
        let mut t: termios = unsafe { mem::zeroed() };
        let rc = unsafe { tcgetattr(fd, &mut t as *mut termios) };
        if rc != 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(t)
        }
    }

    /// Small helper: set termios for a given fd.
    fn set_termios(fd: i32, t: &termios) -> io::Result<()> {
        let rc = unsafe { tcsetattr(fd, TCSANOW, t as *const termios) };
        if rc != 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    /// RAII guard that disables canonical mode and echo, restoring on drop.
    struct RawMode {
        fd: i32,
        saved: termios,
    }

    impl RawMode {
        fn new(fd: i32) -> io::Result<Self> {
            let mut current = get_termios(fd)?;
            let saved = current;

            // Disable canonical mode and echo.
            current.c_lflag &= !(ICANON | ECHO);

            // Optionally, we could tweak VMIN/VTIME for finer control,
            // but we’ll preserve existing semantics.
            set_termios(fd, &current)?;

            Ok(Self { fd, saved })
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            // Best effort restore; nothing to do on failure in Drop.
            let _ = set_termios(self.fd, &self.saved);
        }
    }

    /// Read a single key assuming we are already in raw/no-echo mode.
    fn read_key_raw() -> io::Result<Key> {
        let mut buffer = [0u8; 3];

        // We read up to 3 bytes so we can catch arrow escape sequences.
        let n = std::io::stdin().read(&mut buffer)?;

        if n == 0 {
            // EOF or nothing read — treat as unknown
            return Ok(Key::Unknown);
        }

        match buffer[0] {
            27 => {
                // Escape sequence: try to match ESC [ A/B/C/D
                // Only proceed if we actually got those extra bytes.
                if n >= 3 && buffer[1] == b'[' {
                    match buffer[2] {
                        b'A' => Ok(Key::ArrowUp),
                        b'B' => Ok(Key::ArrowDown),
                        b'C' => Ok(Key::ArrowRight),
                        b'D' => Ok(Key::ArrowLeft),
                        _ => Ok(Key::Unknown),
                    }
                } else if n == 1 {
                    Ok(Key::Escape)
                } else {
                    Ok(Key::Unknown)
                }
            }
            b'\n' => Ok(Key::Enter),
            b'\t' => Ok(Key::Tab),
            127 => Ok(Key::Backspace),
            c => Ok(Key::Char(c as char)),
        }
    }

    // Reads a key from the console, temporarily switching to raw/no-echo.
    pub(crate) fn read_key() -> io::Result<Key> {
        let _rm = RawMode::new(STDIN_FILENO)?;
        read_key_raw()
    }

    /// Wait up to `timeout` for a key. Returns Some(key) if pressed, None on timeout.
    /// Echo is disabled during the wait so nothing is visually printed.
    pub(super) fn key_pressed_within(timeout: Duration) -> io::Result<Option<Key>> {
        let _rm = RawMode::new(STDIN_FILENO)?;

        let mut fds = pollfd {
            fd: STDIN_FILENO,
            events: POLLIN,
            revents: 0,
        };

        // Clamp to i32::MAX safely.
        let ms = timeout.as_millis().min(i32::MAX as u128) as i32;

        let rc = unsafe { poll(&mut fds as *mut pollfd, 1, ms) };

        if rc < 0 {
            return Err(io::Error::last_os_error());
        }
        if rc == 0 {
            // timeout — nothing was pressed
            return Ok(None);
        }

        // Read one key while still in raw mode (guard restores on drop).
        match read_key_raw() {
            Ok(k) => Ok(Some(k)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}
