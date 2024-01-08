// Code for managing styles used by ASS/SSA files
// https://fileformats.fandom.com/wiki/SubStation_Alpha#Styles_section

// Example of .ass Styles section:
// [V4+ Styles]
// Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
// Style: Default,LTFinnegan Medium,52,&H00FFFFFF,&H000000FF,&H00000000,&HC0000000,0,0,0,0,100,100,0,0,1,2,1.5,2,110,110,30,1
// Style: signs,VAGRounded BT,45,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,0,8,80,80,30,1
// Style: Title,昭和モダン体,40,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,0,2,10,10,10,1
// Style: Chalkboard,Eraser,45,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,0,8,80,80,30,1

use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

use thiserror::Error;

use super::assa_colour::{AssaColour, MalformedColourError};

#[derive(Error, Debug, PartialEq)]
pub enum MalformedStyleError {
    #[error("malformed AssaColour string")]
    AssaColourError(MalformedColourError),
    #[error("malformed Style string")]
    FormatError,
    #[error("could not parse style value")]
    ParseError,
}

impl From<MalformedColourError> for MalformedStyleError {
    fn from(error: MalformedColourError) -> Self {
        MalformedStyleError::AssaColourError(error)
    }
}

impl From<ParseIntError> for MalformedStyleError {
    fn from(_error: ParseIntError) -> Self {
        MalformedStyleError::ParseError
    }
}

impl From<ParseFloatError> for MalformedStyleError {
    fn from(_error: ParseFloatError) -> Self {
        MalformedStyleError::ParseError
    }
}

#[derive(Default, Debug)]
pub struct Style {
    name: String,
    fontname: String,
    fontsize: u8,
    primary_colour: AssaColour,
    secondary_colour: AssaColour,
    outline_colour: AssaColour,
    back_colour: AssaColour,
    // For bold. italic, underline, and strike_out -1 is True, 0 is False (see ass specification)
    bold: bool,
    italic: bool,
    underline: bool,
    strike_out: bool,
    scale_x: u16,
    scale_y: u16,
    spacing: f32,
    angle: f32,
    border_style: u8,
    outline: f32,
    shadow: f32,
    alignment: u8,
    margin_l: u16,
    margin_r: u16,
    margin_v: u16,
    encoding: u8,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Style: {},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.name,
            self.fontname,
            self.fontsize,
            self.primary_colour,
            self.secondary_colour,
            self.outline_colour,
            self.back_colour,
            match self.bold {
                true => "-1",
                false => "0",
            },
            match self.italic {
                true => "-1",
                false => "0",
            },
            match self.underline {
                true => "-1",
                false => "0",
            },
            match self.strike_out {
                true => "-1",
                false => "0",
            },
            self.scale_x,
            self.scale_y,
            self.spacing,
            self.angle,
            self.border_style,
            self.outline,
            self.shadow,
            self.alignment,
            self.margin_l,
            self.margin_r,
            self.margin_v,
            self.encoding,
        )
    }
}

impl FromStr for Style {
    type Err = MalformedStyleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        if !s.starts_with("Style: ") {
            return Err(MalformedStyleError::FormatError);
        }
        // split_at: "Style: " is 7 characters
        let style_values: Vec<&str> = s.split_at(7).1.split(',').collect();
        // There are 23 fields in a v4+ style (Name, Fontname, Fontsi...)
        if style_values.len() != 23 {
            return Err(MalformedStyleError::FormatError);
        }

        Ok(Self {
            name: style_values[0].to_string(),
            fontname: style_values[1].to_string(),
            fontsize: style_values[2].parse()?,
            primary_colour: AssaColour::from_str(style_values[3])?,
            secondary_colour: AssaColour::from_str(style_values[4])?,
            outline_colour: AssaColour::from_str(style_values[5])?,
            back_colour: AssaColour::from_str(style_values[6])?,
            bold: style_values[7] == "-1",
            italic: style_values[8] == "1",
            underline: style_values[9] == "1",
            strike_out: style_values[10] == "1",
            scale_x: style_values[11].parse()?,
            scale_y: style_values[12].parse()?,
            spacing: style_values[13].parse()?,
            angle: style_values[14].parse()?,
            border_style: style_values[15].parse()?,
            outline: style_values[16].parse()?,
            shadow: style_values[17].parse()?,
            alignment: style_values[18].parse()?,
            margin_l: style_values[19].parse()?,
            margin_r: style_values[20].parse()?,
            margin_v: style_values[21].parse()?,
            encoding: style_values[22].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        let valid_style_string = "Style: Default,LTFinnegan Medium,52,&H00FFFFFF,&H000000FF,&H00000000,&HC0000000,-1,0,0,0,100,100,0,0,1,2,1.5,2,110,110,30,1";
        let colour_error_string = "Style: Default,LTFinnegan Medium,52,00FFFFFF,&H000000FF,&H00000000,&HC0000000,0,0,0,0,100,100,0,0,1,2,1.5,2,110,110,30,1";
        let colour_error_string_2 = "Style: Default,LTFinnegan Medium,52,&H00FFXFFF,&H000000FF,&H00000000,&HC0000000,0,0,0,0,100,100,0,0,1,2,1.5,2,110,110,30,1";
        let format_error_string = "Style: LTFinnegan Medium,52,&H00FFFFFF,&H000000FF,&H00000000,&HC0000000,0,0,0,0,100,100,0,0,1,2,1.5,2,110,110,30,1";
        let parse_error_string = "Style: Default,LTFinnegan Medium,VERY_LARGE,&H00FFFFFF,&H000000FF,&H00000000,&HC0000000,0,0,0,0,100,100,0,0,1,2,1.5,2,110,110,30,1";

        assert!(Style::from_str(valid_style_string).is_ok());
        assert_eq!(
            Style::from_str(colour_error_string).unwrap_err(),
            MalformedStyleError::AssaColourError(MalformedColourError)
        );
        assert_eq!(
            Style::from_str(colour_error_string_2).unwrap_err(),
            MalformedStyleError::AssaColourError(MalformedColourError)
        );
        assert_eq!(
            Style::from_str(format_error_string).unwrap_err(),
            MalformedStyleError::FormatError
        );
        assert_eq!(
            Style::from_str(parse_error_string).unwrap_err(),
            MalformedStyleError::ParseError
        );
    }
}
