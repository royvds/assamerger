use regex::Regex;
use strsim::normalized_levenshtein;

pub fn remove_styling(text: &str) -> String {
    let re = Regex::new(r#"\{.*?\}"#).unwrap();
    re.replace_all(text, "").to_string().trim().to_string()
}

pub fn clean_spaces(text: impl AsRef<str>) -> String {
    let mut output = String::from(text.as_ref());

    while output.contains("  ") {
        output = output.replace("  ", " ");
    }

    output.trim().to_string()
}

pub fn prep_for_value_measuring(text: impl AsRef<str>) -> String {
    let mut output = remove_styling(text.as_ref().into());
    output = output.replace(r#"\N"#, " ").replace("\"", "");
    output = clean_spaces(&output);
    output.to_string()
}

pub fn prep_for_value_measuring_batch(text_list: &Vec<impl AsRef<str>>) -> Vec<String> {
    text_list
        .iter()
        .map(|t| prep_for_value_measuring(t))
        .collect()
}

pub fn prep_for_distance_measuring(text: impl AsRef<str>) -> String {
    let mut output = remove_styling(text.as_ref().into());
    output = output.replace(r#"\N"#, " ");
    output = clean_spaces(&output);

    output
        .to_lowercase()
        .replace(|c| !"abcdefghijklmnopqrstuvwxyz ".contains(c), "")
}

fn split_indices(text: impl AsRef<str>) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::with_capacity(7);
    for mat in Regex::new(r"[?!,.:;]+").unwrap().find_iter(text.as_ref()) {
        if mat.start() == 0 || mat.end() == text.as_ref().len() {
            continue;
        } else {
            indices.push(mat.end());
        }
    }

    let newline_indices: Vec<usize> = text.as_ref().match_indices(r#"\N"#).map(|m| m.0).collect();
    for i in newline_indices {
        if !indices.contains(&i) && !indices.contains(&(i - 1)) {
            indices.push(i + 2)
        }
    }

    indices.sort();
    indices
}

pub fn split_count(event_text: impl AsRef<str>) -> usize {
    split_indices(event_text).len() + 1
}

fn string_split_prep(text: impl AsRef<str>) -> String {
    let mut output = String::from(text.as_ref());

    output = output.replace(r#"\N"#, " ").trim().to_string();
    while output.contains("  ") {
        output = output.replace("  ", " ");
    }
    output
}

pub fn split_groups(event_text: impl AsRef<str>) -> Vec<Vec<String>> {
    // splits on all split_values and combines it into all possible
    // combinations for which a line could've been split.

    let split_indices = split_indices(&event_text);
    let split_value_count = split_indices.len();
    let possible_combinations = 1 << split_value_count;
    let mut result: Vec<Vec<String>> = Vec::new();

    for i in 1..possible_combinations {
        let mut last: Option<usize> = None;
        let mut current: Vec<String> = Vec::new();

        for j in 0..split_value_count {
            if !((i & (1 << j)) > 0) {
                continue;
            }
            current.push(match last {
                None => string_split_prep(&event_text.as_ref()[0..split_indices[j]]),
                Some(m) => {
                    string_split_prep(&event_text.as_ref()[split_indices[m]..split_indices[j]])
                }
            });
            last = Some(j);
        }

        current.push(match last {
            None => string_split_prep(&event_text.as_ref()[0..]),
            Some(m) => string_split_prep(&event_text.as_ref()[split_indices[m]..]),
        });

        result.push(current);
    }

    result.sort();
    result.dedup();
    result
}

pub fn levenshtein_ratio(a: &str, b: &str) -> f64 {
    normalized_levenshtein(
        &prep_for_distance_measuring(a),
        &prep_for_distance_measuring(b),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_indices_works() {
        assert_eq!(
            split_indices(
                r#"...w-what? \NAre you insane?!!\NNo, I am not! You are\Nsimply overreacting..."#,
            ),
            vec![10, 30, 35, 45, 55]
        );
    }

    #[test]
    fn split_groups_works() {
        assert_eq!(
            split_groups(r#"Hey, how are you?\N I'm fine thank you."#),
            vec![
                vec!["Hey,", "how are you?", "I'm fine thank you."],
                vec!["Hey,", "how are you? I'm fine thank you."],
                vec!["Hey, how are you?", "I'm fine thank you."],
            ],
        );
    }
}
