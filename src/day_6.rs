pub fn day_6() {
    let input = include_str!("A:/advent/input_day6.txt");

    part1(input);
    part2(input);
}

fn part2(input: &str) {
    let mut lines_rev = input.lines().rev().skip_while(|line| line.is_empty());
    let operators_line = lines_rev.next().unwrap();

    let mut operators: Vec<(char, usize)> = vec![];
    let mut counter = 0;
    let mut operation = '*';
    for char in operators_line.chars().skip(1) {
        if !char.is_whitespace() {
            operators.push((operation, counter));

            operation = char;
            counter = 0;
        } else { counter += 1; }
    }

    operators.push((operation, counter + 1));

    let lines: Vec<&str> = lines_rev.by_ref().collect::<Vec<&str>>().into_iter().rev().collect();
    let total_lines = lines.len();

    let mut result = 0;
    let mut begin = 0;
    operators.iter().for_each(|(operation, size)| {
        let mut nums = vec![];
        for i in 0..total_lines {
            let num = &lines[i][begin..begin + size];
            nums.push(num);
        }

        begin += size + 1;

        let nums = transpose(nums.iter().map(|x| x.chars().collect()).collect())
            .iter()
            .map(|x| x.iter()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<u64>().unwrap()
            ).collect::<Vec<u64>>();
        match operation {
            '*' => result += nums.iter().product::<u64>(),
            '+' => result += nums.iter().sum::<u64>(),
            _ => panic!("Unknown operation")
        }
    });

    println!("[Day 6] Part 2: {}", result);
}

fn part1(input: &str) {
    let mut is_numbers = true;

    let mut lists_of_numbers: Vec<Vec<u64>> = Vec::new();
    let mut results: Vec<u64> = Vec::new();

    input.trim()
        .lines()
        .for_each(|line| {
            if is_numbers && line.starts_with('*')  { is_numbers = false; }

            let split = line.split_whitespace();
            if is_numbers {
                split.enumerate().for_each(|(idx, value)| {
                    if lists_of_numbers.len() <= idx {
                        lists_of_numbers.push(Vec::new());
                    }

                    lists_of_numbers[idx].push(value.parse::<u64>().unwrap());
                });
            } else {
                split.enumerate().for_each(|(idx, op)| {
                    let acc = lists_of_numbers[idx].iter().fold(if op == "*" {1} else {0}, |acc, x| match op {
                        "*" => acc * x,
                        "+" => acc + x,
                        _ => panic!("Unknown operation"),
                    });

                    results.push(acc);
                });
            }
        });

    println!("[Day 6] Part 1: {}", results.iter().sum::<u64>());
}

fn transpose<T : Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() { return Vec::new(); }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![Vec::with_capacity(rows); cols];

    for r in 0..rows {
        for c in 0..cols {
            transposed[c].push(matrix[r][c]);
        }
    }

    transposed
}