use assa_parse::assa_file::event::Event;

pub fn filter_events_by_style<'events>(
    events: &'events Vec<Event>,
    styles: &Vec<String>,
    keep_comments: bool,
) -> Vec<&'events Event> {
    events
        .iter()
        .filter(|event| (keep_comments || !event.comment) && styles.contains(&event.style))
        .collect()
}

pub fn get_text_of_events<'a>(events: &'a [&Event]) -> Vec<&'a String> {
    events.iter().map(|event| &event.text).collect()
}
