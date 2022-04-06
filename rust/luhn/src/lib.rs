/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let s: String = code.chars().filter(|c| c.is_digit(10)).collect();
    let double_element = |e: u32| if 2 * e > 9 { (2 * e) - 9 } else { 2 * e };

    //check for length & invalid symbols
    if s.len() <= 1 || code.chars().any(|c| !c.is_digit(10) && c != ' ') {
        return false;
    } else {
        let sum: u32 = s
            .chars()
            .rev()
            .map(|val| val.to_digit(10).unwrap())
            .enumerate()
            .map(|(i, val)| if i % 2 == 1 { double_element(val) } else { val })
            .sum();

        match (sum) % 10 {
            0 => true,
            _ => false,
        }
    }
}
