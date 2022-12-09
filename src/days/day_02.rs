use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn day_02_part1(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);
    let play_scores: HashMap<char, usize> =
        HashMap::from([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);
    let mut final_score: usize = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();

        if !current_line.is_empty() {
            let split: Vec<&str> = current_line.split(' ').collect();
            let elve_choice = char::from_str(split[0]).unwrap();
            let my_choice = char::from_str(split[1]).unwrap();

            let elve_score = play_scores[&elve_choice];
            let my_score = play_scores[&my_choice];
            let diff = my_score.abs_diff(elve_score);
            match diff {
                2 => {
                    if my_score > elve_score {
                        final_score += 0
                    } else {
                        final_score += 6
                    }
                }
                1 => {
                    if my_score > elve_score {
                        final_score += 6
                    } else {
                        final_score += 0
                    }
                }
                0 => final_score += 3,
                _ => final_score += 0,
            }
            final_score += my_score;
        }
    }
    final_score
}

pub fn day_02_part2(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);
    let play_scores: HashMap<char, usize> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    let mut final_score: usize = 0;

    for line in reader.lines() {
        let current_line = line.unwrap();

        if !current_line.is_empty() {
            let split: Vec<&str> = current_line.split(' ').collect();
            let elve_choice = char::from_str(split[0]).unwrap();
            let my_outcome = char::from_str(split[1]).unwrap();

            let elve_score = play_scores[&elve_choice];

            match my_outcome {
                'X' => {
                    //I need to lose
                    if elve_score > 1 {
                        final_score += elve_score - 1;
                    } else {
                        final_score += 3;
                    }
                }
                'Y' => {
                    //I need to draw
                    final_score += 3 + elve_score;
                }
                'Z' => {
                    if elve_score < 3 {
                        final_score += 6 + elve_score + 1;
                    } else {
                        final_score += 6 + 1;
                    }
                }
                _ => final_score += 0,
            }
        }
    }
    final_score
}
