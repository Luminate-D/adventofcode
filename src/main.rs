use std::time::Instant;
use crate::day_1::day_1;
use crate::day_2::day_2;

mod day_1;
mod day_2;

fn main() {
    println!("Hello, baka!");

    let now = Instant::now();
    // day_1();
    day_2();

    let elapsed = now.elapsed();

    println!("Time elapsed: {:?}", elapsed);
}
