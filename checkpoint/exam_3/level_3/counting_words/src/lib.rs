use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    let processed_words = words.to_lowercase().chars().map(|c| {
        if c.is_alphanumeric() || c == '\'' || c.is_whitespace() {
            c
        } else {
            ' ' 
        }
    })
    .collect::<String>();

    for word in  processed_words.split_whitespace() {
        let cleaned_word =word.trim_matches('\'');

        if !cleaned_word.is_empty() {
            *word_count.entry(cleaned_word.to_string())
            .or_insert(0) += 1;
        }            
    }
    word_count
}

