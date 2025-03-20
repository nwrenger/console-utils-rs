//! Style Utilities
//!
//! This module provides functions for coloring text using ANSI escape code sequences.
//! It allows setting foreground and background colors, as well as applying bold, italic,
//! underline, blink, reverse and strikethrough formatting.

use std::fmt;

/// Represents all colors the text/background can be.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// Black color.
    Black,
    /// Red color.
    Red,
    /// Green color.
    Green,
    /// Yellow color.
    Yellow,
    /// Blue color.
    Blue,
    /// Magenta color.
    Magenta,
    /// Cyan color.
    Cyan,
    /// White color.
    White,
    /// Bright Black color.
    BrightBlack,
    /// Bright Red color.
    BrightRed,
    /// Bright Green color.
    BrightGreen,
    /// Bright Yellow color.
    BrightYellow,
    /// Bright Blue color.
    BrightBlue,
    /// Bright Magenta color.
    BrightMagenta,
    /// Bright Cyan color.
    BrightCyan,
    /// Bright White color.
    BrightWhite,
    /// An ANSI color of your choice.
    ANSI(u8),
}

impl Color {
    /// Converts a color to its ANSI foreground variant.
    fn fg_code(self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
            Color::ANSI(c) => c,
        }
    }

    /// Converts a color to its ANSI background variant.
    fn bg_code(self) -> u8 {
        self.fg_code() + 10
    }
}

/// Represents a piece of text with optional color and formatting.
pub struct StyledText<'a> {
    text: &'a str,
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    reverse: bool,
    strikethrough: bool,
}

impl<'a> StyledText<'a> {
    /// Creates a new `StyledText` instance with default settings.
    ///
    /// # Arguments
    /// * `text` - The string slice representing the text.
    ///
    /// # Returns
    /// A `StyledText` instance with no colors or formatting applied.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            strikethrough: false,
        }
    }

    /// Sets the foreground color of the text.
    ///
    /// # Arguments
    /// * `color` - A `Color` enum variant representing the desired foreground color.
    ///
    /// # Returns
    /// The modified `StyledText` instance.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color of the text.
    ///
    /// # Arguments
    /// * `color` - A `Color` enum variant representing the desired background color.
    ///
    /// # Returns
    /// The modified `StyledText` instance.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Enables bold formatting for the text.
    ///
    /// # Returns
    /// The modified `StyledText` instance with bold formatting applied.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enables italic formatting for the text.
    ///
    /// # Returns
    /// The modified `StyledText` instance with italic formatting applied.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Enables underline formatting for the text.
    ///
    /// # Returns
    /// The modified `StyledText` instance with underline formatting applied.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Enables blink effect for the text.
    ///
    /// # Returns
    /// The modified `StyledText` instance with blinking enabled.
    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

    /// Enables reverse video (inverts foreground and background colors).
    ///
    /// # Returns
    /// The modified `StyledText` instance with inverted colors.
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Enables strikethrough formatting for the text.
    ///
    /// # Returns
    /// The modified `StyledText` instance with strikethrough applied.
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Converts the colored text into a formatted ANSI escape sequence string.
    ///
    /// # Returns
    /// A `String` containing the ANSI-formatted text.
    pub fn format_sequence(&'a self) -> String {
        let mut codes = Vec::new();
        if let Some(fg) = self.fg {
            codes.push(fg.fg_code());
        }
        if let Some(bg) = self.bg {
            codes.push(bg.bg_code());
        }
        if self.bold {
            codes.push(1);
        }
        if self.italic {
            codes.push(3);
        }
        if self.underline {
            codes.push(4);
        }
        if self.blink {
            codes.push(5);
        }
        if self.reverse {
            codes.push(7);
        }
        if self.strikethrough {
            codes.push(9);
        }

        let codes_str = codes
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(";");

        if !codes.is_empty() {
            format!("\x1B[{}m{}\x1B[0m", codes_str, self.text)
        } else {
            self.text.to_string()
        }
    }
}

/// Implements the `Display` trait for `StyledText`, allowing it to be printed directly.
impl fmt::Display for StyledText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format_sequence())
    }
}
