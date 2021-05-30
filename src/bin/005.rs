// 2520 is the smallest number that can be
// divided by each of the numbers from 1 to 10
// without any remainder.
// What is the smallest positive number that is
// evenly divisible by all of the numbers from 1 to 20?

fn is_divisible(number: i32, other: i32) -> bool {
    number % other == 0
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
    let mut number: i32 = 1;
    let prompt: i32 = 20;

    loop {
        number += 1;
        let mut i: i32 = 1;
        loop {
            // println!("{} is {}/prompt", number, i);
            if !is_divisible(number, i) || i == prompt {
                break;
            } else {
                if i == prompt / 2 {
                    println!("{} is {}/{}!", number, i, prompt);
                }
                i += 1;
            }
        }
        if i == prompt {
            println!("Answer: {}", number);
            std::process::exit(0);
        }
    }
}

fn main() {
    solve();
}
