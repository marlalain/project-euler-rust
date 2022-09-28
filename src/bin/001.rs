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
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn all_natural_numbers_below(n: i32) -> i32 {
	(3..n)
		.filter(|x| x % 3 == 0 || x % 5 == 0)
		.sum()
}

fn solve() {
	let sum = all_natural_numbers_below(1000);
	println!("{}", sum);
}