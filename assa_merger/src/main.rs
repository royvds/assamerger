use ass_comp::alignment::align_events;
use ass_comp::event_processor::filter_events_by_style;
use assa_parse::assa_file::event::Event;
use assa_parse::assa_file::AssaFile;

fn main() {
    // let original_file =
    //     "[Final8] Mirai Nikki - 01 (BD 10-bit 1920x1080 x264 FLAC)[965F3FEB]_Subtitles01.UND.ass";
    // let modified_file = "[FFF] Mirai Nikki - 01 [BD][1080p-FLAC][AD0D9174]_Track03.ass";

    let original_file: &str =
        "[Final8] Mirai Nikki - 02 (BD 10-bit 1920x1080 x264 FLAC)[7F11AAB2]_Subtitles01.UND.ass";
    let modified_file = "[FFF] Mirai Nikki - 02 [BD][1080p-FLAC][E2BCE249]_Track03.ass";

    let mut original = AssaFile::from_file(original_file).expect("Failed to parse file");
    let mut modified = AssaFile::from_file(modified_file).expect("Failed to parse file");

    original.events.sort_by(|a, b| a.start.cmp(&b.start));
    modified.events.sort_by(|a, b| a.start.cmp(&b.start));

    let dialogue_styles: Vec<String> = Vec::from([
        String::from("Default"),
        String::from("Alternate"),
        String::from("DefaultAlt"),
    ]);

    let original_dialogue_events: Vec<&Event> =
        filter_events_by_style(&original.events, &dialogue_styles, false);

    let modified_dialogue_events: Vec<&Event> =
        filter_events_by_style(&modified.events, &dialogue_styles, false);

    // original_dialogue_events.drain(40..original_dialogue_events.len());
    // modified_dialogue_events.drain(40..modified_dialogue_events.len());

    align_events(&original_dialogue_events, &modified_dialogue_events, 4);
}

// pub fn main() {
//     let original = "Hey there, my name is Thomas. How are you?";
//     let potential_splits = [
//         "Hey there.",
//         "I'm Thomas,",
//         "how are you?",
//         "I'm Henry!",
//         "And I am doing excellent!",
//     ];

//     // let (best_levenshtein, matching_split) = match_split_line(original, &potential_splits);
//     let (best_levenshtein, matching_merge) = match_merged_lines(&potential_splits, original);

//     println!("{}", matching_merge);
//     println!("{}", best_levenshtein);
// }
