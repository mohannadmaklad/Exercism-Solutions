pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let num_of_digits = digits.len() as u32;
    digits
        .iter()
        .fold(0, |acc, &d| acc + ((d as u32).pow(num_of_digits)))
        == num
}
