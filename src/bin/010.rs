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

#[test]
fn does_it_solve() {
    solve();
}

#[test]
fn is_less_than_time_limit() {
    use std::time::SystemTime;
    let now = SystemTime::now();
    solve();
    match now.elapsed() {
        Ok(elapsed) => {
            if elapsed.as_secs() > 60 {
                panic!("Shouldn't take that long");
            }
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

fn solve() {
    let mut sum: i64 = 0;
    for n in 2..2000000 {
        if is_prime(n) {
            sum += n;
        }
    }
    println!("Answer: {}", sum);
}

fn main() {
    solve();
}
