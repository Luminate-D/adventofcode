pub fn day_2() {
    let input = include_str!("A:/advent/input_day2.txt");

    let ranges: Vec<(u64, u64)> = input.split(",").map(|range| {
        if range.is_empty() { return (0, 0); }

        range.trim()
            .split_once('-')
            .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap())).unwrap()
    }).collect();

    part1(&ranges);
    part2(&ranges);
}

fn part1(ranges: &Vec<(u64, u64)>) {
    let mut invalid_ids: Vec<u64> = vec![];
    for &(low, high) in ranges {
        let mut low = low;

        // Invalid ids cant have an odd number of digits
        // => skip blocks of powers of 10, if number has odd digits number
        while low <= high {
            let magnitude = low.ilog10();
            let block_end = 10_u64.pow(magnitude + 1) - 1;

            if magnitude % 2 == 0 {
                low = block_end + 1;
                continue;
            }

            // Notice that part 1 suggests invalid ids MUST have divisor of form 10^(d/2) + 1
            // eg: 123123 [d = 6] is 123 * 1001, where 1001 = 10^(6/2) + 1
            let high = block_end.min(high);
            for i in low..=high {
                let digits = i.ilog10() + 1;
                if i % (10_u64.pow(digits / 2) + 1) == 0 {
                    invalid_ids.push(i);
                }
            }

            low = high + 1;
        }
    }

    let answer: u64 = invalid_ids.iter().sum();
    println!("[Day 2] Part1: {}", answer);
}

fn part2(ranges: &Vec<(u64, u64)>) {
    let mut invalid_ids: Vec<u64> = vec![];
    for &(low, high) in ranges {
        for i in low..=high {
            if part2_is_invalid(i) { invalid_ids.push(i); }
        }
    }

    let answer: u64 = invalid_ids.iter().sum();
    println!("[Day 2] Part2: {}", answer)
}

fn part2_is_invalid(n: u64) -> bool {
    let s= n.to_string();
    let digits = s.len();

    for block_size in 1..=digits/2 {
        if !digits.is_multiple_of(block_size) { continue; }

        let blocks = digits / block_size;
        let block = &s[0..block_size];
        let mut ok = true;

        for i in 1..blocks {
            if &s[i*block_size .. (i+1)*block_size] != block {
                ok = false;
                break;
            }
        }

        if ok { return true; }
    }

    false
}