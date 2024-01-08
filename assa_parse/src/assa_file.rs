use crate::assa_file::{
    event::{Event, MalformedEventError},
    project_garbage::ProjectGarbage,
    script_info::ScriptInfo,
    style::{MalformedStyleError, Style},
};
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    str::FromStr,
};
use thiserror::Error;

use self::{project_garbage::MalformedProjectGarbageError, script_info::MalformedScriptInfoError};

pub mod assa_colour;
pub mod event;
pub mod project_garbage;
pub mod script_info;
pub mod style;

#[derive(Error, Debug)]
pub enum MalformedAssaFileError {
    #[error("malformed ScriptInfo")]
    ScriptInfoError(MalformedScriptInfoError),
    #[error("malformed ProjectGarbage")]
    ProjectGarbageError(MalformedProjectGarbageError),
    #[error("malformed Style string")]
    StyleError(MalformedStyleError),
    #[error("malformed Event string")]
    EventError(MalformedEventError),
}

impl From<MalformedScriptInfoError> for MalformedAssaFileError {
    fn from(error: MalformedScriptInfoError) -> Self {
        MalformedAssaFileError::ScriptInfoError(error)
    }
}

impl From<MalformedProjectGarbageError> for MalformedAssaFileError {
    fn from(error: MalformedProjectGarbageError) -> Self {
        MalformedAssaFileError::ProjectGarbageError(error)
    }
}

impl From<MalformedEventError> for MalformedAssaFileError {
    fn from(error: MalformedEventError) -> Self {
        MalformedAssaFileError::EventError(error)
    }
}

impl From<MalformedStyleError> for MalformedAssaFileError {
    fn from(error: MalformedStyleError) -> Self {
        MalformedAssaFileError::StyleError(error)
    }
}

#[derive(Default)]
pub struct AssaFile {
    pub script_info: ScriptInfo,
    pub styles: Vec<Style>,
    pub events: Vec<Event>,
    pub project_garbage: ProjectGarbage,
    pub aegisub_extradata: String,
}

impl AssaFile {
    pub fn from_file(ass_file_path: &str) -> Result<AssaFile, MalformedAssaFileError> {
        let mut assa_file = AssaFile::default();
        let file_content = fs::read_to_string(ass_file_path).expect("Could not read file.");

        let re = Regex::new(r"(?s)(?m)\[(.*?)\](.*?)(\n\n|\r\n\r\n|\z)").unwrap(); // Matches data blocks (e.g. [Script Info], [Styles], [Events], [Aegisub Project Garbage])

        for cap in re.captures_iter(&file_content) {
            println!("cap: '{}'", &cap[1]);
            match &cap[1] {
                "Script Info" => assa_file.script_info = ScriptInfo::from_str(cap[0].trim())?,
                "V4+ Styles" => assa_file.styles = parse_styles(cap[0].trim())?,
                "Events" => assa_file.events = parse_events(cap[0].trim())?,
                "Aegisub Project Garbage" => {
                    assa_file.project_garbage = ProjectGarbage::from_str(cap[0].trim())?
                }
                "Aegisub Extradata" => assa_file.aegisub_extradata = cap[0].trim().to_string(),
                _ => println!("Unknown section: {}", &cap[1]),
            }
        }

        Ok(assa_file)
    }
}

impl AssaFile {
    pub fn save_file_as(&self, output_filepath: &str) {
        let f = File::create(output_filepath).expect("fc");
        let mut writer = BufWriter::new(f);

        let data = format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}",
            self.script_info,
            self.project_garbage,
            styles_to_string(&self.styles),
            events_to_string(&self.events),
            self.aegisub_extradata,
        );

        writer
            .write_all(data.as_bytes())
            .expect("Unable to write data");
    }
}

fn parse_events(events_string: &str) -> Result<Vec<Event>, MalformedEventError> {
    let mut events: Vec<Event> = Vec::with_capacity(events_string.lines().count() - 2);
    // .skip(2) because first two lines are:
    // [Events]
    // Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
    for line in events_string.trim().lines().skip(2) {
        events.push(Event::from_str(line)?);
    }
    Ok(events)
}

fn parse_styles(styles_string: &str) -> Result<Vec<Style>, MalformedStyleError> {
    let mut styles: Vec<Style> = Vec::with_capacity(styles_string.lines().count() - 2);
    // .skip(2) because first two lines are:
    // [V4+ Styles]
    // Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
    for line in styles_string.trim().lines().skip(2) {
        styles.push(Style::from_str(line)?);
    }
    Ok(styles)
}

fn events_to_string(events: &Vec<Event>) -> String {
    let mut export_string = String::with_capacity(90 + (events.len() * 160));

    export_string.push_str(
        r#"[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text"#,
    );

    for event in events {
        export_string.push_str(&format!("\n{}", event))
    }

    export_string
}

fn styles_to_string(styles: &Vec<Style>) -> String {
    let mut export_string = String::with_capacity(250 + (styles.len() * 150));

    export_string.push_str(r#"[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding"#);

    for style in styles {
        export_string.push_str(&format!("\n{}", style))
    }

    export_string
}
