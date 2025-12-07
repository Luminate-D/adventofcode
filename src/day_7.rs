use std::collections::{HashMap, HashSet};

pub fn day_7() {
    let input = include_str!("A:/advent/input_day7.txt");

    part1(input);
    part2(input);
}

// Idea - track beams
// If meet splitter - remove middle beam, add left and right beams
// Use hashset so overlapping beams are merged
pub fn part1(input: &str) {
    let mut lines = input.lines();
    let mut beams: HashSet<usize> = HashSet::new();

    beams.insert(lines.next().unwrap().len() / 2);

    let mut splits = 0usize;
    for line in lines {
        let mut new_beams = HashSet::new();
        let chars = line.chars().collect::<Vec<char>>();

        for &beam_pos in beams.iter() {
            match chars[beam_pos] {
                '^' => {
                    new_beams.insert(beam_pos - 1);
                    new_beams.insert(beam_pos + 1);
                    splits += 1;
                }

                _ => { new_beams.insert(beam_pos); }
            }
        }

        beams = new_beams;
    }

    println!("[Day 7] Part 1: {}", splits);
}

// Idea almost the same
// Track beams AND timelines
// begin with 1 timeline and 1 beam
// when beam meets splitter - each of new beams clones the timeline (each of new beams has same number of timelines leading there)
// when beams overlap - each beam timelines are summed (cuz new beam has all timelines that lead to it)
// at the end - count timelines on the last row
pub fn part2(input: &str) {
    let mut lines = input.lines();
    let mut timelines: HashMap<usize, usize> = HashMap::new();

    timelines.insert(lines.next().unwrap().len() / 2, 1);

    let mut total_timelines = 1usize;
    for line in lines {
        let mut new_timelines = HashMap::new();
        let chars = line.chars().collect::<Vec<char>>();

        for (&pos, &count) in timelines.iter() {
            match chars[pos] {
                '^' => {
                    *new_timelines.entry(pos - 1).or_insert(0) += count;
                    *new_timelines.entry(pos + 1).or_insert(0) += count;

                    total_timelines += count;
                }

                _ => { *new_timelines.entry(pos).or_insert(0) += count; }
            }
        }

        timelines = new_timelines;
    }

    println!("[Day 7] Part 2: {}", total_timelines);
}