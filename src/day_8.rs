use std::collections::{BinaryHeap, HashSet};

pub fn day_8() {
    let input = include_str!("A:/advent/input_day8.txt");

    part1(input);
    part2(input);
}

struct Objedenitel {
    // each group of points is a hashset with their indices
    groups: Vec<HashSet<usize>>
}

impl Objedenitel {
    fn new() -> Self {
        Self {
            groups: vec![]
        }
    }

    // merge two points
    fn objedenit(&mut self, i: usize, j: usize) {
        let mut merged = vec![];

        // find groups that points belong to
        for (idx, group) in self.groups.iter().enumerate() {
            if group.contains(&i) || group.contains(&j) {
                merged.push(idx);
            }
        }

        // if no groups => create one with both points
        if merged.is_empty() {
            let mut hs = HashSet::new();
            hs.insert(i);
            hs.insert(j);
            self.groups.push(hs);
        } else {
            // if groups found => merge them and add both points to the new group
            // this handles both cases if one point is in group, and if both are in different groups
            let mut new_group = HashSet::new();
            for &idx in merged.iter().rev() {
                // remove group from the list and extend new group
                // iterating in reverse to avoid shifting indices
                let group = self.groups.swap_remove(idx);
                new_group.extend(group);
            }

            new_group.insert(i);
            new_group.insert(j);

            // add new merged group to the list
            self.groups.push(new_group);
        }
    }

    fn top3(&self) -> Vec<usize> {
        let mut sizes: Vec<usize> = self.groups.iter().map(|g| g.len()).collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes.truncate(3);
        sizes
    }
}

// Idea is
// we anyways need to know top 1000 points that are closest to each other
// make a heap of size 1000 with (distance, point)
// if new distance is smaller than the largest in heap, pop largest and insert new
fn part1(input: &str) {
    let points = input.lines().map(|line| {
        let split: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
        (split[0], split[1], split[2])
    }).collect::<Vec<(i64, i64, i64)>>();

    // (distance, points[idx1], points[idx2])
    let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();

    let distance_squared = |p1: (i64, i64, i64), p2: (i64, i64, i64)| -> i64 {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        let dz = p1.2 - p2.2;
        dx * dx + dy * dy + dz * dz
    };

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = distance_squared(points[i], points[j]);

            if heap.len() < 1000 { heap.push((dist, i, j)); }
            else if dist < heap.peek().unwrap().0 {
                heap.pop();
                heap.push((dist, i, j));
            }
        }
    }

    // merge all pairs of top 1000 closest points
    let mut objedenitel = Objedenitel::new();
    for (_, i, j) in heap.into_sorted_vec().iter() {
        objedenitel.objedenit(*i, *j);
    }

    // get top 3 groups' sizes of points & multiply them
    println!("Part 1: {:?}", objedenitel.top3().iter().product::<usize>());
}

fn part2(input: &str) {
    
}