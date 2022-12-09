use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn day_01_part1(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);
    let mut current_elve: usize = 0;
    let mut elve_values: Vec<usize> = vec![0];

    for line in reader.lines() {
        let current_line = line.unwrap();

        if !current_line.is_empty() {
            let current_val: usize = usize::from_str(&current_line).unwrap();
            elve_values[current_elve] += current_val;
        } else {
            elve_values.push(0);
            current_elve += 1;
        }
    }
    elve_values.iter().max().unwrap().to_owned()
}

pub fn day_01_part2(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);

    let mut current_elve: usize = 0;
    let mut elve_values: Vec<usize> = vec![0];

    for line in reader.lines() {
        let current_line = line.unwrap();

        if !current_line.is_empty() {
            let current_val: usize = usize::from_str(&current_line).unwrap();
            elve_values[current_elve] += current_val;
        } else {
            elve_values.push(0);
            current_elve += 1;
        }
    }

    elve_values.sort();
    elve_values.iter().rev().take(3).sum()
}
