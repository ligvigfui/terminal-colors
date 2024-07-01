use crate::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    RGB(RGB),
    U8(U8Color),
    Default,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub(crate) fn to_foreground(&self) -> String {
        match self {
            Color::Black => "\x1B[30m".to_string(),
            Color::Red => "\x1B[31m".to_string(),
            Color::Green => "\x1B[32m".to_string(),
            Color::Yellow => "\x1B[33m".to_string(),
            Color::Blue => "\x1B[34m".to_string(),
            Color::Magenta => "\x1B[35m".to_string(),
            Color::Cyan => "\x1B[36m".to_string(),
            Color::White => "\x1B[37m".to_string(),
            Color::RGB(rgb) => format!("\x1B[38;2;{};{};{}m", rgb.red, rgb.green, rgb.blue),
            Color::U8(u8_color) => format!("\x1B[38;5;{}m", u8_color.color),
            Color::Default => "\x1B[39m".to_string(),
            Color::BrightBlack => "\x1B[90m".to_string(),
            Color::BrightRed => "\x1B[91m".to_string(),
            Color::BrightGreen => "\x1B[92m".to_string(),
            Color::BrightYellow => "\x1B[93m".to_string(),
            Color::BrightBlue => "\x1B[94m".to_string(),
            Color::BrightMagenta => "\x1B[95m".to_string(),
            Color::BrightCyan => "\x1B[96m".to_string(),
            Color::BrightWhite => "\x1B[97m".to_string(),
        }
    }

    pub(crate) fn to_background(&self) -> String {
        match self {
            Color::Black => "\x1B[40m".to_string(),
            Color::Red => "\x1B[41m".to_string(),
            Color::Green => "\x1B[42m".to_string(),
            Color::Yellow => "\x1B[43m".to_string(),
            Color::Blue => "\x1B[44m".to_string(),
            Color::Magenta => "\x1B[45m".to_string(),
            Color::Cyan => "\x1B[46m".to_string(),
            Color::White => "\x1B[47m".to_string(),
            Color::RGB(rgb) => format!("\x1B[48;2;{};{};{}m", rgb.red, rgb.green, rgb.blue),
            Color::U8(u8_color) => format!("\x1B[48;5;{}m", u8_color.color),
            Color::Default => "\x1B[49m".to_string(),
            Color::BrightBlack => "\x1B[100m".to_string(),
            Color::BrightRed => "\x1B[101m".to_string(),
            Color::BrightGreen => "\x1B[102m".to_string(),
            Color::BrightYellow => "\x1B[103m".to_string(),
            Color::BrightBlue => "\x1B[104m".to_string(),
            Color::BrightMagenta => "\x1B[105m".to_string(),
            Color::BrightCyan => "\x1B[106m".to_string(),
            Color::BrightWhite => "\x1B[107m".to_string(),
        }
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Color::RGB(rgb) => {
                match &rgb.name {
                    Some(name) => write!(f, "{}", name),
                    None => write!(f, "RGB({},{},{})", rgb.red, rgb.green, rgb.blue),
                }
            },
            Color::U8(u8_c) => {
                match &u8_c.name {
                    Some(name) => write!(f, "{}", name),
                    None => write!(f, "U8({})", u8_c.color),
                }
            },
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_foreground() {
        let color = Color::Red;
        assert_eq!(color.to_foreground(), "\x1B[31m".to_string());
    }

    #[test]
    fn to_background() {
        let color = Color::Red;
        assert_eq!(color.to_background(), "\x1B[41m".to_string());
    }

    #[test]
    fn to_string() {
        assert_eq!(Color::Red.to_string(), "Red".to_string());
        assert_eq!(Color::RGB(RGB::named("White", 255, 255, 255)).to_string(), "White".to_string());
        assert_eq!(Color::RGB(RGB::new(255, 255, 255)).to_string(), "RGB(255,255,255)".to_string());
        assert_eq!(Color::U8(U8Color::named("White", 255)).to_string(), "White".to_string());
        assert_eq!(Color::U8(U8Color::new(255)).to_string(), "U8(255)".to_string());
    }
}