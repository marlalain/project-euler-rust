// Problem 6: Sum square difference
// The sum of the squares of the first ten natural numbers is,
//      1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,
//      (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten
// natural numbers and the square of the sum is 3025 - 385 = 2640.
// Find the difference between the sum of the squares of the first one
// hundred natural numbers and the square of the sum.

fn sum_of_squares(number: i64) -> i64 {
    let mut sum: i64 = 0;
    for n in 0..number + 1 {
        sum += n * n;
    }

    sum
}

fn square_of_sum(number: i64) -> i64 {
    let mut sum: i64 = 0;
    for n in 0..number + 1 {
        sum += n;
    }

    sum * sum
}

fn main() {
    println!(
        "{} - {} = {}",
        sum_of_squares(100),
        square_of_sum(100),
        square_of_sum(100) - sum_of_squares(100)
    );
}
