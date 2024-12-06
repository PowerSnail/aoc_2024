use std::{collections::HashMap, u64};

use aoc_2024;

fn main() {
    let part = aoc_2024::get_part();
    let input = aoc_2024::get_stdin();
    let mut a1 = vec![];
    let mut a2 = vec![];
    for line in input.split("\n") {
        let numbers: Vec<_> = line.split_whitespace().collect();
        a1.push(numbers[0].parse::<u64>().unwrap());
        a2.push(numbers[1].parse::<u64>().unwrap());
    }

    if part == 1 {
        a1.sort();
        a2.sort();
        let mut sum: u64 = 0;
        for i in 0..a1.len() {
            sum += a1[i].abs_diff(a2[i]);
        }
        println!("{}", sum);
    } else {
        let mut counts = HashMap::new();
        for n in a2 {
            let c = counts.get(&n).unwrap_or(&0);
            counts.insert(n, c + 1);
        }
        let mut sum: u64 = 0;
        for n in a1 {
            sum += n * counts.get(&n).unwrap_or(&0);
        }
        println!("{}", sum);
    }
}
