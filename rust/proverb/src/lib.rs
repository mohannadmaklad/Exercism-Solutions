pub fn build_proverb(list: &[&str]) -> String {
    let mut text = String::new();
    if list.is_empty() {
        return text;
    }
    for i in 0..list.len() - 1 {
        text.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i],
            list[i + 1]
        ));
    }
    text.push_str(&format!(
        "And all for the want of a {}.",
        list.first().unwrap()
    ));
    text
}
