// text

fn is_prime(number: i64) -> bool {
    if number == 1 {
        return false;
    }

    let mut i: i64 = 2;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn main() {
    let mut sum: i64 = 0;
    for n in 2..2000000 {
        if is_prime(n) {
            sum += n;
        }
    }
    println!("Answer: {}", sum);
}
