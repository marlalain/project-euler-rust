fn check(limit: i32, prompt: i32) -> bool {
    for a in 2..limit {
        for b in 2..limit {
            for c in 2..limit {
                if a + b + c == prompt && (a * a) + (b * b) == (c * c) {
                    println!("\n{}+{}+{} = {}", a, b, c, prompt);
                    println!("{}*{}*{} = {}", a, b, c, a * b * c);
                    return true;
                }
            }
        }
    }
    false
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
    let prompt: i32 = 1000;
    let mut limit: i32 = 10;
    loop {
        if !check(limit, prompt) {
            limit += 10;
        } else {
            break;
        }
    }
}

fn main() {
    solve();
}
