pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let s2 = String::from(s2);
    let mut distance: usize = 0;
    for (i, c) in s1.chars().enumerate() {
        if s2.as_bytes()[i] as char != c {
            distance = distance + 1;
        }
    }
    Some(distance)
}
