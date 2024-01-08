// Code for managing Aegisub Project Garbage of ASS/SSA files
// Wiki fileformats below notes the project garabage properties
// under the script info, this is outdated information
// https://fileformats.fandom.com/wiki/SubStation_Alpha#Script_Info_section

// Example of .ass Project Garbage section:
// [Aegisub Project Garbage]
// Last Style Storage: Default
// Audio File: ?video
// Video File: ..\..\..\mirai01_premux.mkv
// Video Position: 32031

use core::fmt;
use std::{num::ParseIntError, str::FromStr};

use log;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MalformedProjectGarbageError {
    #[error("could not parse project garbage value")]
    ParseError,
}

impl From<ParseIntError> for MalformedProjectGarbageError {
    fn from(_error: ParseIntError) -> Self {
        MalformedProjectGarbageError::ParseError
    }
}

pub struct ProjectGarbage {
    pub last_style_storage: Option<String>,
    pub audio_file: Option<String>,
    pub video_file: Option<String>,
    pub video_position: Option<u16>,
    pub video_zoom_percent: Option<f64>,
}

impl Default for ProjectGarbage {
    fn default() -> ProjectGarbage {
        ProjectGarbage {
            last_style_storage: Some(String::from("Default")),
            audio_file: None,
            video_file: None,
            video_position: None,
            video_zoom_percent: Some(1.0),
        }
    }
}

impl fmt::Display for ProjectGarbage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_data = String::with_capacity(160); // Roughly the amount of bytes when using all parameters

        string_data.push_str("[Aegisub Project Garbage]\n");
        if self.last_style_storage.is_some() {
            string_data.push_str(&format!(
                "Last Style Storage: {}\n",
                self.last_style_storage.as_ref().unwrap()
            ));
        }
        if self.audio_file.is_some() {
            string_data.push_str(&format!(
                "Audio File: {}\n",
                self.audio_file.as_ref().unwrap()
            ));
        }
        if self.video_file.is_some() {
            string_data.push_str(&format!(
                "Video File: {}\n",
                self.video_file.as_ref().unwrap()
            ));
        }
        if self.video_position.is_some() {
            string_data.push_str(&format!(
                "Video Position: {}\n",
                self.video_position.as_ref().unwrap()
            ));
        }
        write!(f, "{}", string_data.trim())
    }
}

impl FromStr for ProjectGarbage {
    type Err = MalformedProjectGarbageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut script_info = ProjectGarbage::default();
        for line in s.trim().lines().skip(1) {
            // Skipping [Aegisub Project Garbage]
            let (line_property, line_value) = line.split_once(": ").unwrap();

            match line_property {
                "Last Style Storage" => {
                    script_info.last_style_storage = Some(String::from(line_value))
                }
                "Audio File" => script_info.audio_file = Some(String::from(line_value)),
                "Video File" => script_info.video_file = Some(String::from(line_value)),
                "Video Position" => {
                    script_info.video_position = Some(line_value.trim().parse().unwrap())
                }
                "Video Zoom Percent" => {
                    script_info.video_zoom_percent = Some(line_value.trim().parse().unwrap())
                }
                &_ => log::warn!(
                    "Ignoring unknown [Aegisub Project Garbage] property ({})",
                    line_property
                ),
            }
        }
        Ok(script_info)
    }
}
