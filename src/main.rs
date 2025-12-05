use std::time::Instant;
use crate::day_1::day_1;
use crate::day_2::day_2;
use crate::day_3::day_3;
use crate::day_4::day_4;
use crate::day_5::day_5;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    println!("Hello, baka!");

    let now = Instant::now();
    // day_1();
    // day_2();
    // day_3();
    // day_4();
    day_5();
    
    let elapsed = now.elapsed();

    println!("Time elapsed: {:?}", elapsed);
}
