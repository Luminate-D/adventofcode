pub fn day_4() {
    let input = include_str!("A:/advent/input_day4.txt");

    solve(input, true);
    solve(input, false);
}

fn solve(input: &str, part_1: bool) {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut counter = 0;

    loop {
        let mut mutated = false;
        let mut temp_grid = grid.clone();

        for y in 0..height {
            for x in 0..width {
                if !grid[y][x] {
                    continue;
                }

                let mut bomb_neighbors = 0;
                for dy in [-1isize, 0, 1] {
                    for dx in [-1isize, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        // oob check
                        if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                            continue;
                        }

                        if grid[ny as usize][nx as usize] {
                            bomb_neighbors += 1;
                        }
                    }
                }

                if bomb_neighbors < 4 {
                    temp_grid[y][x] = false;
                    counter += 1;
                    mutated = true;
                }
            }
        }

        if part_1 {
            break;
        }

        grid = temp_grid;
        if !mutated {
            break;
        }
    }

    println!("[Day 4] Part {}: {}", if part_1 { "1" } else { "2" }, counter);
}
