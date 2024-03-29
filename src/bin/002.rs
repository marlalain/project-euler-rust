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
// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
//
//      1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

fn fib(n: u128) -> u128 {
	fn _fib(l: u128, r: u128, n: u128) -> u128 {
		if n < 1 {
			r
		} else {
			_fib(r, l + r, n - 1)
		}
	}

	_fib(0, 1, n)
}

fn even_sum_fib_until(n: u128) -> u128 {
	let mut sum: u128 = 0;
	let mut x: u128 = 1;

	loop {
		let f = fib(x);
		if f > n {
			break;
		} else if f % 2 == 0 {
			sum += f;
		}

		x += 1;
	}

	sum
}

fn solve() {
	let a = even_sum_fib_until(4000000);

	println!("{:#?}", a);
}