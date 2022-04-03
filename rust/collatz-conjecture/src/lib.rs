use std::cmp::Ordering;

pub fn collatz(mut n: u64) -> Option<u64> {
    let mut sum = 0;
    if n <= 0 {
        return None;
    }

    while n != 1 {
        match n % 2 {
            0 => n = n / 2,
            _ => match n.cmp(&(&(&std::u64::MAX / 3) - 1)) {
                Ordering::Less => n = (3 * n) + 1,
                _ => return None,
            },
        }
        sum = sum + 1;
    }
    Some(sum)
}
