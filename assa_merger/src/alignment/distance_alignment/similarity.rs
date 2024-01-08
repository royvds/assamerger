use crate::alignment::text_processor::levenshtein_ratio;
use std::cmp::min;

use super::super::text_processor::split_groups;

pub fn calc_split_similarity<'a>(line: &str, splits: &Vec<&'a String>) -> (f64, Vec<&'a String>) {
    let potential_splits = split_groups(line);
    let mut result: (f64, Vec<&String>) = (0f64, vec![]);

    for potential_split in potential_splits {
        let split_count = potential_split.len();
        let mut total_levenshtein_ratio = 0f64;

        for i in 0..min(split_count, splits.len()) {
            total_levenshtein_ratio += levenshtein_ratio(&potential_split[i], splits[i]);
        }

        let mean_levenshtein_ratio = total_levenshtein_ratio / split_count as f64;
        if mean_levenshtein_ratio > result.0 {
            result.0 = mean_levenshtein_ratio;
            result.1 = splits[0..potential_split.len()].to_vec();
        }
    }

    result
}

pub fn calc_merge_similarity<'a>(
    line_parts: &Vec<&'a String>,
    merged_line: &str,
) -> (f64, Vec<&'a String>) {
    // Returns into how many lines have been merged into the merged_line
    let mut merged_nr_of_lines = 0;
    let line_parts_count = line_parts.len();
    let mut potential_merge = String::with_capacity(line_parts_count);
    potential_merge.push_str(line_parts[0]);
    let mut highest_levenshtein_ratio = levenshtein_ratio(merged_line, &potential_merge);

    for i in 1..line_parts_count {
        potential_merge.push_str(line_parts[i]);
        let l_ratio = levenshtein_ratio(&potential_merge, merged_line);

        if l_ratio > highest_levenshtein_ratio {
            highest_levenshtein_ratio = l_ratio;
            merged_nr_of_lines += 1;
        }
    }

    (
        highest_levenshtein_ratio,
        line_parts[0..merged_nr_of_lines].to_vec(),
    )
}
