pub mod semantic_similarity;
mod similarity;

use std::cmp::min;

use super::super::event_processor::get_text_of_events;
use super::text_processor::{prep_for_value_measuring, split_count};
use super::{AlignmentAction, ComparisonContext};
use assa_parse::assa_file::event::Event;
use semantic_similarity::SemanticSimilarity;
use similarity::{calc_merge_similarity, calc_split_similarity};

pub fn semantic_alignment(
    context: &mut ComparisonContext,
    semantic_similarity: &SemanticSimilarity,
) -> (bool, f64) {
    let prev_action_was_a_split = match context.prev_alignment_action {
        AlignmentAction::Split => true,
        _ => false,
    };

    let (
        current_similarity,
        (split_similarity, split_similarities, split_lines),
        (merge_similarity, merge_lines),
        prev_split_similarity,
        prev_split_merge_similarity,
        prev_similarity,
        next_similarity,
    ) = get_similarities(
        *context.comparison_loop_index,
        *context.offset,
        *context.prev_offset,
        context.lookahead,
        context.original_events,
        context.modified_events,
        context.prev_alignment_action,
        semantic_similarity,
    );

    let similarities: Vec<f64> = vec![current_similarity, split_similarity, merge_similarity];

    let maximum_similarity = similarities
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .to_owned();

    let split_max_similarity = match split_similarities.len() > 1 {
        true => split_similarities
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .to_owned(),
        false => 0f64,
    };

    let do_println = context.original_events[*context.comparison_loop_index].text
        == r#"Running right through the minefield? Is he an idiot?"#;

    if do_println {
        println!(
            "similarities: {} || {} || {} || {} || {} || {} || {}",
            current_similarity,
            split_similarity,
            merge_similarity,
            prev_similarity,
            next_similarity,
            prev_split_similarity,
            prev_split_merge_similarity
        );
        println!("split similarities: {:?}", split_similarities);
    }

    if maximum_similarity >= 0.3 {
        if merge_similarity >= 0.8 && split_similarity >= 0.8 {
            println!("semantic: split sentence kept as split");
            println!("");
            *context.comparison_loop_index += 2;
            *context.prev_alignment_action = AlignmentAction::None;
            return (true, (merge_similarity + split_similarity) / 2f64);
        } else if next_similarity > maximum_similarity * 1.3 {
            println!("semantic: modified addition");
            println!("");
            *context.prev_offset = *context.offset;
            *context.offset += 1;
            // *context.comparison_loop_index += 1;
            *context.prev_alignment_action = AlignmentAction::Next;
            return (true, next_similarity);
        } else if prev_similarity > maximum_similarity
            && prev_action_was_a_split
            && prev_split_merge_similarity > prev_split_similarity
        {
            println!("semantic: prev detected");
            println!("");
            *context.offset -= 1;
            *context.comparison_loop_index += 1;
            *context.prev_alignment_action = AlignmentAction::Prev;
            return (true, prev_similarity);
        } else if split_similarities.len() > 1
            && split_max_similarity > maximum_similarity
            && split_similarity > maximum_similarity - 0.2
        {
            println!("semantic: split");
            println!("");
            *context.prev_offset = *context.offset;
            *context.offset += split_lines.len() as i32 - 1;
            *context.comparison_loop_index += 1;
            *context.prev_alignment_action = AlignmentAction::Split;
            return (true, split_similarity);
        }

        if maximum_similarity == current_similarity
            || maximum_similarity - current_similarity <= 0.005
            || (current_similarity >= 0.8 && maximum_similarity - current_similarity <= 0.1)
        {
            return (false, current_similarity);
        } else if maximum_similarity == split_similarity {
            println!("semantic: split");
            println!("");
            *context.prev_offset = *context.offset;
            *context.offset += split_lines.len() as i32 - 1;
            *context.comparison_loop_index += 1;
            *context.prev_alignment_action = AlignmentAction::Split;
            return (true, split_similarity);
        } else if maximum_similarity == merge_similarity
            && merge_similarity - current_similarity >= 0.1
        {
            println!("semantic: merged");
            println!("");
            if do_println {
                println!("lines: {:?}", merge_lines);
            }
            *context.prev_offset = *context.offset;
            *context.offset -= merge_lines.len() as i32 - 1;
            *context.comparison_loop_index += merge_lines.len();
            *context.prev_alignment_action = AlignmentAction::Merge;
            return (true, merge_similarity);
        }
    }

    if prev_similarity > 0.3 && prev_similarity > next_similarity {
        if let AlignmentAction::Split = context.prev_alignment_action {
            if prev_split_merge_similarity > prev_split_similarity {
                println!("semantic: prev detected");
                println!("");
                *context.offset -= 1;
                *context.comparison_loop_index += 1;
                *context.prev_alignment_action = AlignmentAction::Prev;
                return (true, prev_similarity);
            }
        }
    } else if next_similarity > 0.3 {
        println!("semantic: modified addition");
        println!("");
        *context.prev_offset = *context.offset;
        *context.offset += 1;
        // *context.comparison_loop_index += 1;
        *context.prev_alignment_action = AlignmentAction::Next;
        return (true, merge_similarity);
    }

    (false, current_similarity)
}

fn get_similarities<'a>(
    index: usize,
    offset: i32,
    prev_offset: i32,
    lookahead: usize,
    original_events: &'a Vec<&'a Event>,
    modified_events: &'a Vec<&'a Event>,
    prev_alignment_action: &'a mut AlignmentAction,
    semantic_similarity: &SemanticSimilarity,
) -> (
    f64,
    (f64, Vec<f64>, Vec<String>),
    (f64, Vec<&'a String>),
    f64,
    f64,
    f64,
    f64,
) {
    let original_event = original_events[index];
    let modified_event = modified_events[(index as i32 + offset) as usize];
    let original_text = &original_event.text;
    let modified_text = &modified_event.text;

    let current_similarity = semantic_similarity.compare(
        &prep_for_value_measuring(original_text),
        &prep_for_value_measuring(modified_text),
    );

    let (split_similarity, split_similarities, split_lines) = calc_split_similarity(
        &original_text,
        &get_text_of_events(
            &modified_events[(index as i32 + offset) as usize
                ..(index as i32 + offset) as usize + min(split_count(&original_text), lookahead)],
        ),
        semantic_similarity,
    );

    let mut minimal_merge_potential = 1;
    let single_merge_len = original_text.len() + original_events[index + 1].text.len() + 1;
    if modified_text.len().abs_diff(single_merge_len)
        < modified_text.len().abs_diff(original_text.len())
    {
        minimal_merge_potential = 2;
    }
    let mut original_split_count = split_count(&original_text);
    let modified_split_count = split_count(&modified_text);
    if original_text.ends_with("...") || original_text.ends_with(",") {
        original_split_count += 1;
    }

    let max_merge_count = [
        &minimal_merge_potential,
        &original_split_count,
        &modified_split_count,
    ]
    .iter()
    .max()
    .unwrap()
    .to_owned()
    .to_owned();

    let potential_merge_count = min(max_merge_count, lookahead);
    let (merge_similarity, merge_lines) = calc_merge_similarity(
        &get_text_of_events(&original_events[index..index + potential_merge_count]),
        &modified_text,
        semantic_similarity,
    );

    let prev_similarity = match index as i32 + offset {
        0 => 0f64,
        _ => semantic_similarity.compare(
            &prep_for_value_measuring(original_text),
            &prep_for_value_measuring(&modified_events[(index as i32 + offset - 1) as usize].text),
        ),
    };

    let next_similarity = semantic_similarity.compare(
        &prep_for_value_measuring(original_text),
        &prep_for_value_measuring(&modified_events[(index as i32 + offset + 1) as usize].text),
    );

    let mut prev_split_similarity = 0f64;
    let mut prev_split_merge_similarity = 0f64;
    if let AlignmentAction::Split = prev_alignment_action {
        let prev_split_lines = calc_split_similarity(
            &original_events[index - 1].text,
            &get_text_of_events(
                &modified_events[(index as i32 - 1 + prev_offset) as usize
                    ..(index as i32 - 1 + prev_offset) as usize
                        + min(split_count(&original_events[index - 1].text), lookahead)],
            ),
            semantic_similarity,
        )
        .2;

        let prev_modified_text = &modified_events[(index as i32 + offset - 1) as usize].text;
        let last_split_item = &prev_split_lines[prev_split_lines.len() - 1];
        let mut potential_merge =
            String::with_capacity(original_text.len() + last_split_item.len());

        potential_merge.push_str(&last_split_item);
        potential_merge.push_str(" ");
        potential_merge.push_str(&original_text);

        prev_split_similarity = semantic_similarity.compare(&last_split_item, prev_modified_text);

        prev_split_merge_similarity =
            semantic_similarity.compare(&potential_merge, prev_modified_text)
    }

    (
        current_similarity,
        (split_similarity, split_similarities, split_lines),
        (merge_similarity, merge_lines),
        prev_split_similarity,
        prev_split_merge_similarity,
        prev_similarity,
        next_similarity,
    )
}
