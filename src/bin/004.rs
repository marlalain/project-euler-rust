#[test]
fn does_it_solve() {
	solve();
}

fn main() {
	solve();
}

#[test]
fn is_less_than_time_limit() {
	use std::time::SystemTime;
	let now = SystemTime::now();
	solve();
	match now.elapsed() {
		Ok(elapsed) => {
			println!("took {}ms", elapsed.as_millis());
			if elapsed.as_secs() > 60 {
				panic!("shouldn't take that long");
			}
		}
		Err(e) => {
			panic!("{:?}", e);
		}
	}
}

// ---
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(n: i64) -> bool {
	let s = n.to_string();
	s.chars().rev().collect::<String>() == s
}

fn until_palindrome(x: i64, y: i64) -> i64 {
	let mut result: i64 = 0;
	for i in (100..x).rev() {
		for j in (100..y).rev() {
			let r = i * j;
			if is_palindrome(r) && r > result {
				result = r;
			}
		}
	}

	result
}

fn solve() {
	println!("{}", until_palindrome(999, 999));
}