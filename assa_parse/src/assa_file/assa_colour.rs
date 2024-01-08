// Code for managing colour construct used by ASS/SSA files
// https://fileformats.fandom.com/wiki/SubStation_Alpha#Data_types
// https://www.matroska.org/technical/subtitles.html#ssaass-subtitles

use std::{fmt, str::FromStr};

#[derive(Default, Debug)]
pub struct AssaColour {
    /// Alpha channel is inverted, 255 = completely transparent and 0 is no transparency
    alpha: Option<u8>,
    red: u8,
    green: u8,
    blue: u8,
}

impl AssaColour {
    pub fn to_abgr(&self) -> String {
        format!(
            "&H{:02X}{:02X}{:02X}{:02X}",
            self.alpha.unwrap_or(0),
            self.blue,
            self.green,
            self.red
        )
    }

    pub fn to_bgr(&self) -> String {
        format!("&H{:02X}{:02X}{:02X}", self.blue, self.green, self.red)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MalformedColourError;

impl fmt::Display for MalformedColourError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse string to AssColour: input is not a valid AssColour representation (&H + ABGR or &H + BGR)")
    }
}

impl fmt::Display for AssaColour {
    /// Returns &H RGBA or RGB depending on whether alpha channel is set
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.alpha {
            Some(alpha) => write!(
                f,
                "&H{:02X}{:02X}{:02X}{:02X}",
                alpha, self.blue, self.green, self.red
            ),
            None => write!(f, "&H{:02X}{:02X}{:02X}", self.blue, self.green, self.red),
        }
    }
}

impl FromStr for AssaColour {
    type Err = MalformedColourError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_length = s.len();
        if !s.starts_with("&H") {
            return Err(MalformedColourError);
        }
        if hex_length == 10 {
            let alpha = u8::from_str_radix(&s[2..4], 16).map_err(|_| MalformedColourError)?;
            let blue = u8::from_str_radix(&s[4..6], 16).map_err(|_| MalformedColourError)?;
            let green = u8::from_str_radix(&s[6..8], 16).map_err(|_| MalformedColourError)?;
            let red = u8::from_str_radix(&s[8..10], 16).map_err(|_| MalformedColourError)?;

            Ok(Self {
                alpha: Some(alpha),
                blue,
                green,
                red,
            })
        } else if hex_length == 8 {
            let blue = u8::from_str_radix(&s[2..4], 16).map_err(|_| MalformedColourError)?;
            let green = u8::from_str_radix(&s[4..6], 16).map_err(|_| MalformedColourError)?;
            let red = u8::from_str_radix(&s[6..8], 16).map_err(|_| MalformedColourError)?;
            Ok(Self {
                alpha: None,
                blue,
                green,
                red,
            })
        } else {
            Err(MalformedColourError)
        }
    }
}
