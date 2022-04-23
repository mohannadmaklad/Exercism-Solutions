pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => generate_primes(n),
    }
}

fn is_prime(n: u32) -> bool {
    let max: u32 = (n as f64).sqrt() as u32;
    for i in 2..max + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
fn generate_primes(n: u32) -> u32 {
    //Won't check for max. prime as it's not necessary for the exercise
    let mut added_num = 1;
    let mut prime = 0;
    let mut i = 1;

    while added_num < n {
        prime = (6 * i) - 1;
        if is_prime(prime) {
            added_num = added_num + 1;
            if added_num == n {
                break;
            }
        }

        prime = (6 * i) + 1;
        if is_prime(prime) {
            added_num = added_num + 1;
            if added_num == n {
                break;
            }
        }
        i = i + 1;
    }
    prime
}
