pub fn day_1() {
    let input = include_str!("A:/advent/input_day1.txt");

    let mut password_1: i32 = 0;
    let mut password_2: i32 = 0;

    let mut dial: i32 = 50;

    input.split("\n").for_each(|line| {
        if line.is_empty() { return; }

        let (sign, num) = match &line[..1] {
            "L" => (-1, &line[1..].parse::<i32>().unwrap()),
            "R" => (1, &line[1..].parse::<i32>().unwrap()),
            _ => panic!("Invalid input")
        };

        let mut buf = sign * num;

        while buf != 0 {
            dial += sign;
            buf -= sign;

            dial = (dial + 100) % 100;
            if dial == 0 { password_2 += 1; }
        }

        if dial == 0 { password_1 += 1; }
    });

    println!("[Day 1] Part1: {} | Part2: {}", password_1, password_2);
}