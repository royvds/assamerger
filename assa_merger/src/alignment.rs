mod distance_alignment;
mod semantic_alignment;
mod text_processor;

use assa_parse::assa_file::event::Event;
use std::cmp::min;

use distance_alignment::text_distance_alignment;
use semantic_alignment::semantic_alignment;
use semantic_alignment::semantic_similarity::SemanticSimilarity;

#[derive(Debug)]
pub enum AlignmentAction {
    None,
    Merge,
    Split,
    Interjection,
    Next,
    Prev,
    Lookahead,
    Lookback,
}

pub struct ComparisonContext<'a> {
    comparison_loop_index: &'a mut usize,
    offset: &'a mut i32,
    prev_offset: &'a mut i32,
    original_events: &'a Vec<&'a Event>,
    modified_events: &'a Vec<&'a Event>,
    lookahead: usize,
    prev_alignment_action: &'a mut AlignmentAction,
}

pub fn align_events(
    original_events: &Vec<&Event>,
    modified_events: &Vec<&Event>,
    lookahead_range: usize,
) {
    let original_max_index = original_events.len() - 1;
    let modified_max_index = modified_events.len() - 1;
    let mut prev_alignment_action = AlignmentAction::None;

    let semantic_similarity = SemanticSimilarity::default();

    let mut offset: i32 = 0;
    let mut prev_offset: i32 = 0;
    let mut comparison_loop_index = 0;
    'comparison_loop: while comparison_loop_index < original_max_index {
        println!(
            // Debug
            "{} || {} ==== {} || {}",
            comparison_loop_index,
            offset,
            &original_events[comparison_loop_index].text,
            &modified_events[(comparison_loop_index as i32 + offset) as usize].text
        );

        if comparison_loop_index as i32 + offset > modified_max_index as i32 {
            comparison_loop_index += 1;
            continue 'comparison_loop;
        }

        let lookahead = min(lookahead_range, original_max_index);

        let mut comparison_context = ComparisonContext {
            comparison_loop_index: &mut comparison_loop_index,
            offset: &mut offset,
            prev_offset: &mut prev_offset,
            original_events,
            modified_events,
            lookahead,
            prev_alignment_action: &mut prev_alignment_action,
        };

        let (distance_took_actions, _) = text_distance_alignment(&mut comparison_context, false);
        if distance_took_actions {
            continue 'comparison_loop;
        }

        let (semantic_took_actions, _) =
            semantic_alignment(&mut comparison_context, &semantic_similarity);
        if semantic_took_actions {
            continue 'comparison_loop;
        }

        *comparison_context.prev_alignment_action = AlignmentAction::None;
        *comparison_context.comparison_loop_index += 1;
    }
}
