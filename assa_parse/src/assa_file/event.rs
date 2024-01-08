// Code for managing events used by ASS/SSA files
// https://fileformats.fandom.com/wiki/SubStation_Alpha#Events_section

// Example of .ass Events section:
// [Events]
// Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
// Comment: 0,0:15:03.42,0:15:05.41,signs,,0,0,0,,{\fay0.38\pos(578,17)\frz359.276}Emergency Exit
// Comment: 0,0:21:03.10,0:21:04.44,Default,,0,0,0,,{ED}
// Comment: 0,0:23:21.42,0:23:25.63,signs,,0,0,0,,{not typeset, but she's reading it. I'm pretty sure the scroll on screen has a signature "-Murmur" or something}
// Dialogue: 0,0:00:02.28,0:00:04.95,Default,,0,0,25,,Huh? That's odd...
// Dialogue: 0,0:00:04.95,0:00:06.23,Default,,0,0,25,,He won't wake up.
// Dialogue: 0,0:00:07.45,0:00:09.36,Default,,0,0,30,,Come on, wake up...
// Dialogue: 0,0:00:09.75,0:00:11.36,Default,,0,0,0,,Wake up, already!
// Dialogue: 0,0:00:20.32,0:00:22.80,Default,,0,0,25,,Daddy... Mommy...

use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

use chrono::{naive::NaiveTime, Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MalformedEventError {
    #[error("malformed event string")]
    FormatError,
    #[error("could not parse event value")]
    ParseError,
}

impl From<ParseIntError> for MalformedEventError {
    fn from(_error: ParseIntError) -> Self {
        MalformedEventError::ParseError
    }
}

impl From<ParseFloatError> for MalformedEventError {
    fn from(_error: ParseFloatError) -> Self {
        MalformedEventError::ParseError
    }
}

impl From<chrono::ParseError> for MalformedEventError {
    fn from(_error: chrono::ParseError) -> Self {
        MalformedEventError::ParseError
    }
}

fn format_time(time: &NaiveTime) -> String {
    let mut time_str = time.to_string();
    time_str.pop();
    time_str.remove(0);
    time_str
}

#[derive(Default, Debug)]
pub struct Event {
    pub comment: bool,
    pub layer: u8,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub style: String,
    pub name: String,
    pub margin_l: u16,
    pub margin_r: u16,
    pub margin_v: u16,
    pub effect: String,
    pub text: String,
}

impl FromStr for Event {
    type Err = MalformedEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let descriptor = s.split(' ').next().unwrap();
        if descriptor != "Dialogue:" && descriptor != "Comment:" {
            return Err(Self::Err::FormatError);
        }

        // splitn: There are 10 fields in a v4+ event (Layer, Start, En...)
        let event_values: Vec<&str> = s.split_at(descriptor.len() + 1).1.splitn(10, ',').collect();
        Ok(Self {
            comment: descriptor == "Comment:",
            layer: event_values[0].parse()?,
            start: NaiveTime::parse_from_str(event_values[1], "%H:%M:%S%.f")?,
            end: NaiveTime::parse_from_str(event_values[2], "%H:%M:%S%.f")?,
            style: event_values[3].to_string(),
            name: event_values[4].to_string(),
            margin_l: event_values[5].parse()?,
            margin_r: event_values[6].parse()?,
            margin_v: event_values[7].parse()?,
            effect: event_values[8].to_string(),
            text: event_values[9].to_string(),
        })
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {},{},{},{},{},{},{},{},{},{}",
            match self.comment {
                true => "Comment",
                false => "Dialogue",
            },
            self.layer,
            format_time(&self.start),
            format_time(&self.end),
            self.style,
            self.name,
            self.margin_l,
            self.margin_r,
            self.margin_v,
            self.effect,
            self.text
        )
    }
}

impl Event {
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}
