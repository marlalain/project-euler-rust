// text

fn check(limit: i32, prompt: i32) {
    for a in 2..limit {
        for b in 2..limit {
            for c in 2..limit {
                if a + b + c == prompt && (a * a) + (b * b) == (c * c) {
                    println!("\n{}+{}+{} = {}", a, b, c, prompt);
                    println!("{}*{}*{} = {}", a, b, c, a * b * c);
                    std::process::exit(0);
                }
            }
        }
    }
}

fn main() {
    let prompt: i32 = 1000;
    let mut limit: i32 = 10;
    loop {
        check(limit, prompt);
        limit += 10;
    }
}
