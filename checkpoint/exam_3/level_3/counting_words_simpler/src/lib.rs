use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut  word_count_map = HashMap::new();

    for word in words.split_whitespace() {
        let n_word = word.trim_matches(|c: char| !c.is_alphanumeric());

        if !n_word.is_empty() {
            *word_count_map.entry(n_word.to_lowercase()).or_insert(0) += 1;
        }
    }
    word_count_map
}