pub fn day_3() {
    let input = include_str!("A:/advent/input_day3.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;
    input.trim().lines()
        .map(|line| line.chars())
        .for_each(|chars| {
            let mut first = 0;
            let mut last = 0;

            let len = chars.as_str().len();
            for (idx, digit) in chars.map(|char| char.to_digit(10).unwrap()).enumerate() {
                if idx == len - 1 {
                    if last < digit { last = digit; }
                    break;
                }

                if first < digit {
                    first = digit;
                    last = 0;
                } else if last < digit {
                    last = digit;
                }
            }

            sum += first * 10 + last;
        });

    println!("[Day 3] Part 1: {}", sum);
}

// The idea is
// We use a stack to build the largest possible 12-digit joltage
// For each digit in the bank:
//   - While the last digit in the stack is smaller than the current one,
//     and we still have enough remaining digits to reach 12 total,
//     pop it
//   - Then push the current digit
// This maximizes the number from left to right
// cuz larger digit always gives bigger number in more significant position
fn part2(input: &str) {
    let mut sum = 0;
    input.trim().lines()
        .map(|line| line.chars())
        .for_each(|chars| {
            let digits = chars.map(|char| char.to_digit(10).unwrap());

            let len = digits.clone().count();
            let mut stack: Vec<u32> = vec![];
            for (idx, digit) in digits.enumerate() {
                let remaining_digits = len - idx - 1;
                while let Some(&last) = stack.last() {
                    if last < digit && (stack.len() + remaining_digits >= 12) { stack.pop(); }
                    else { break; }
                }

                stack.push(digit);
            }

            stack.truncate(12);

            sum += stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        });

    println!("[Day 3] Part 2: {}", sum);
}