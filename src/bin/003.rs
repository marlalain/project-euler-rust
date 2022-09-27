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
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?


fn solve() {
	let mut n: i64 = 600851475143;
	let mut m: i64 = 2;

	while n != m {
		if n % m == 0 {
			n = n / m;
		}

		m += 1;
	}

	println!("{}", m);
}