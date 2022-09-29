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
			println!("\ntook {}μs", elapsed.as_micros());
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

fn is_palindrome(n: u32) -> bool {
	let mut r = n;
	let d1 = r % 10;
	r = (r - d1) / 10;
	let d2 = r % 10;
	r = (r - d2) / 10;
	let d3 = r % 10;
	r = (r - d3) / 10;
	let d4 = r % 10;
	r = (r - d4) / 10;
	let d5 = r % 10;
	r = (r - d5) / 10;
	let d6 = r % 10;

	d1 == d6 && d2 == d5 && d3 == d4
}

fn until_palindrome(x: u32, y: u32) -> u32 {
	let mut result: u32 = 0;
	for i in (100..x).rev() {
		for j in (100..y).rev() {
			let r = i * j;
			if is_palindrome(r) {
				if r > result {
					result = r;
				}
				break;
			}
		}
	}

	result
}

fn solve() {
	print!("{}", until_palindrome(999, 999));
}