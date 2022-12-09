use std::{
    fs::File,
    io::{Seek, SeekFrom},
};

use days::day_01;

mod days {
    pub mod day_01;
}
fn main() {
    let mut day1_file = File::open("src/days/input_day1.txt").unwrap();
    let result1_1 = day_01::day_01_part1(&day1_file);
    println!("Day 1, part1: {}", result1_1);
    day1_file.seek(SeekFrom::Start(0)).unwrap(); //Reset cursor since it has to read the file again
    let result1_2 = day_01::day_01_part2(&day1_file);
    println!("Day 1, part2: {}", result1_2);
}
