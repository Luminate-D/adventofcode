type Interval = (u64, u64);

pub fn day_5() {
    let input = include_str!("A:/advent/input_day5.txt");

    let (ranges, numbers) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<Interval> = ranges.lines().map(|line| {
        let (min, max) = line.split_once("-").unwrap();
        (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap())
    }).collect();

    ranges.sort_by_key(|range| range.0);
    ranges = merge_intervals(ranges);

    let numbers: Vec<u64> = numbers.lines().map(|line| line.parse::<u64>().unwrap()).collect();

    part1(numbers, &ranges);
    part2(&ranges);
}

fn part1(numbers: Vec<u64>, ranges: &[Interval]) {
    let mut fresh = 0usize;

    for number in numbers {
        if is_in_ranges(number, &ranges) { fresh += 1; }
    }

    println!("[Day 4] Part 1: {}", fresh);
}

fn part2(ranges: &[Interval]) {
    let mut total_fresh = 0u64;

    for range in ranges {
        total_fresh += range.1 - range.0 + 1;
    }

    println!("[Day 4] Part 2: {}", total_fresh);
}

fn is_in_ranges(number: u64, ranges: &[Interval]) -> bool {
    ranges.binary_search_by(|i| {
        if i.0 > number { std::cmp::Ordering::Greater }
        else if i.1 < number { std::cmp::Ordering::Less }
        else { std::cmp::Ordering::Equal }
    }).is_ok()
}

fn merge_intervals(intervals_sorted: Vec<Interval>) -> Vec<Interval> {
    let mut merged = Vec::new();
    let mut current = intervals_sorted[0];

    for &interval in &intervals_sorted[1..] {
        if interval.0 <= current.1 {
            current.1 = current.1.max(interval.1);
        } else {
            merged.push(current);
            current = interval;
        }
    }

    merged.push(current);

    merged
}