use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn day_04_part1(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);

    let mut result: usize = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();

        let split: Vec<&str> = current_line.split(',').collect();

        let first_sections: Vec<usize> = split[0]
            .split('-')
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        let second_sections: Vec<usize> = split[1]
            .split('-')
            .map(|s| usize::from_str(s).unwrap())
            .collect();

        let first_elve: HashSet<usize> = (first_sections[0]..=first_sections[1]).collect();
        let second_elve: HashSet<usize> = (second_sections[0]..=second_sections[1]).collect();

        if second_elve.is_subset(&first_elve) || first_elve.is_subset(&second_elve) {
            result += 1;
        }
    }
    result
}

pub fn day_04_part2(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);

    let mut result: usize = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();

        let split: Vec<&str> = current_line.split(',').collect();

        let first_sections: Vec<usize> = split[0]
            .split('-')
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        let second_sections: Vec<usize> = split[1]
            .split('-')
            .map(|s| usize::from_str(s).unwrap())
            .collect();

        let first_elve: HashSet<usize> = (first_sections[0]..=first_sections[1]).collect();
        let second_elve: HashSet<usize> = (second_sections[0]..=second_sections[1]).collect();

        if second_elve.is_subset(&first_elve) || first_elve.is_subset(&second_elve) {
            result += 1;
        } else {
            let total_overlapping = first_elve
                .iter()
                .filter(|&n| second_elve.contains(n))
                .count();
            if total_overlapping > 0 {
                result += 1;
            }
        }
    }
    result
}
