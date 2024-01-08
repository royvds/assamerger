use std::cmp::min;

use super::semantic_similarity::SemanticSimilarity;
use crate::alignment::text_processor::{
    prep_for_value_measuring, prep_for_value_measuring_batch, remove_styling, split_groups,
};

pub fn calc_split_similarity<'a>(
    line: &str,
    splits: &Vec<&'a String>,
    semantic_similarity: &SemanticSimilarity,
) -> (f64, Vec<f64>, Vec<String>) {
    let potential_splits = split_groups(line);
    let split_vectors = semantic_similarity.encode(&prep_for_value_measuring_batch(splits));
    let mut result: (f64, Vec<f64>, Vec<String>) = (0f64, vec![], vec![]);

    for potential_split in potential_splits {
        let split_count = potential_split.len();

        let mut total_similarity = 0f64;
        let potential_split_vectors =
            semantic_similarity.encode(&prep_for_value_measuring_batch(&potential_split));

        let size = min(split_count, splits.len());
        let mut potential_split_similarities: Vec<f64> = Vec::with_capacity(size);
        for i in 0..size {
            let sim =
                semantic_similarity.cosine_distance(&potential_split_vectors[i], &split_vectors[i]);
            potential_split_similarities.push(sim);
            total_similarity += sim;
        }

        let mean_similarity = total_similarity / split_count as f64;
        if mean_similarity > result.0 {
            result.0 = mean_similarity;
            result.1 = potential_split_similarities;
            result.2 = potential_split;
        }
    }

    result
}

pub fn calc_merge_similarity<'a>(
    line_parts: &Vec<&'a String>,
    merged_line: &str,
    semantic_similarity: &SemanticSimilarity,
) -> (f64, Vec<&'a String>) {
    // Returns into how many lines have been merged into the merged_line
    let mut merged_nr_of_lines = 0;
    let line_parts_count = line_parts.len();

    let mut potential_merge = String::with_capacity(line_parts_count);
    potential_merge.push_str(&remove_styling(line_parts[0]));

    let mut highest_similarity = 0f64;
    let merged_line_vector =
        &semantic_similarity.encode(&vec![prep_for_value_measuring(&merged_line)])[0];

    for i in 1..line_parts_count {
        let next_part = remove_styling(line_parts[i]);

        if potential_merge[potential_merge.len() - 3..potential_merge.len()] == *"..."
            && next_part[0..3] == *"..."
        {
            potential_merge = potential_merge[0..potential_merge.len() - 3].to_string();
            potential_merge.push_str(" ");
            potential_merge.push_str(&next_part[3..next_part.len()]);
        } else {
            potential_merge.push_str(" ");
            potential_merge.push_str(&next_part);
        }

        let potential_merge_vector =
            &semantic_similarity.encode(&vec![prep_for_value_measuring(&potential_merge)])[0];
        let similarity =
            semantic_similarity.cosine_distance(&potential_merge_vector, &merged_line_vector);

        if similarity - highest_similarity > 0.1 {
            highest_similarity = similarity;
            merged_nr_of_lines += 1;
        }
    }

    (
        highest_similarity,
        line_parts[0..merged_nr_of_lines + 1].to_vec(),
    )
}
