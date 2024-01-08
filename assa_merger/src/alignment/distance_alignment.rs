mod similarity;

use std::cmp::min;

use super::super::event_processor::get_text_of_events;
use super::text_processor::levenshtein_ratio;
use super::text_processor::split_count;
use super::{AlignmentAction, ComparisonContext};
use assa_parse::assa_file::event::Event;
use similarity::{calc_merge_similarity, calc_split_similarity};

pub fn text_distance_alignment(context: &mut ComparisonContext, self_call: bool) -> (bool, f64) {
    let (current_similarity, (split_similarity, _), (merge_similarity, merge_lines)) =
        get_similarities(
            *context.comparison_loop_index,
            *context.offset,
            context.lookahead,
            context.original_events,
            context.modified_events,
        );

    if current_similarity >= 0.85 {
        *context.comparison_loop_index += 1;
        *context.prev_alignment_action = AlignmentAction::None;
        return (true, current_similarity);
    }

    let similarities: Vec<f64> = vec![current_similarity, split_similarity, merge_similarity];

    let maximum_similarity = similarities
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    if *maximum_similarity >= 0.6 {
        if *maximum_similarity == current_similarity {
            *context.prev_alignment_action = AlignmentAction::None;
            return (false, current_similarity);
        } else if *maximum_similarity == split_similarity {
            if !self_call {
                println!("distance: split");
                println!("");
            }
            *context.prev_offset = *context.offset;
            *context.offset += 1;
            *context.comparison_loop_index += 1;
            *context.prev_alignment_action = AlignmentAction::Split;
            return (true, split_similarity);
        } else if *maximum_similarity == merge_similarity {
            if !self_call {
                println!("distance: merged ({})\n", merge_lines.len());
                println!();
            }
            *context.prev_offset = *context.offset;
            *context.offset -= merge_lines.len() as i32;
            *context.comparison_loop_index += merge_lines.len() + 1;
            *context.prev_alignment_action = AlignmentAction::Merge;
            return (true, merge_similarity);
        }
    }

    (false, current_similarity)
}

fn get_similarities<'a>(
    index: usize,
    offset: i32,
    lookahead: usize,
    original_events: &'a Vec<&'a Event>,
    modified_events: &'a Vec<&'a Event>,
) -> (f64, (f64, Vec<&'a String>), (f64, Vec<&'a String>)) {
    let original_event = original_events[index];
    let modified_event = modified_events[(index as i32 + offset) as usize];
    let original_text = &original_event.text;
    let modified_text = &modified_event.text;

    let current_similarity = levenshtein_ratio(original_text, modified_text);

    let (split_similarity, split_lines) = calc_split_similarity(
        &original_text,
        &get_text_of_events(
            &modified_events[(index as i32 + offset) as usize
                ..(index as i32 + offset) as usize + min(split_count(&original_text), lookahead)],
        ),
    );

    let (merge_similarity, merge_lines) = calc_merge_similarity(
        &get_text_of_events(
            &original_events[index..index + min(split_count(&modified_text), lookahead)],
        ),
        &modified_text,
    );

    (
        current_similarity,
        (split_similarity, split_lines),
        (merge_similarity, merge_lines),
    )
}
