use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let ordered_word = order_string(&word.to_lowercase());
    //
    let same_word = |w1: &str, w2: &str| w1.to_lowercase() == w2.to_lowercase();
    let is_anagram = |w1: &str| order_string(&w1.to_lowercase()) == ordered_word;

    let a: Vec<&str> = possible_anagrams
        .iter()
        .cloned()
        .filter(|&x| !same_word(x, word) && is_anagram(&x))
        .collect();

    a.iter().cloned().collect::<HashSet<&str>>()
}

pub fn order_string(a: &str) -> String {
    let mut s: Vec<char> = a.chars().collect();
    s.sort_by(|a, b| a.cmp(b));

    String::from_iter(s)
}
