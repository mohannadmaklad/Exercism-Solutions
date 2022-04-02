pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-', '_'])
        .filter(|word| word.len() > 0)
        .into_iter()
        .filter_map(|word| {
            if word.to_uppercase() == word {
                Some(word.to_lowercase())
            } else {
                Some(String::from(word))
            }
        })
        .map(|word| map_word(word))
        .collect()
}

fn map_word(word: String) -> String {
    word.chars()
        .enumerate()
        .filter(|(i, c)| *i == 0 || c.is_uppercase())
        .map(|(_, c)| c)
        .collect::<String>()
        .to_uppercase()
}
