pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut s = Vec::new();
    if len > digits.len() {
        return s;
    }
    for i in 0..digits.len() - len + 1 {
        s.push(digits.chars().skip(i).take(len).collect());
    }
    s
}
