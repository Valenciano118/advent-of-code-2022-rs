use std::{
    fs::File,
    io::{Seek, SeekFrom},
};

use days::day_01;
use days::day_02;
use days::day_03;

mod days {
    pub mod day_01;
    pub mod day_02;
    pub mod day_03;
}
fn main() {
    let pos = SeekFrom::Start(0);
    // DAY 1
    let mut day1_file = File::open("src/days/input_day1.txt").unwrap();
    let result1_1 = day_01::day_01_part1(&day1_file);
    println!("Day 1, part1: {}", result1_1);
    day1_file.seek(pos).unwrap(); //Reset cursor since it has to read the file again
    let result1_2 = day_01::day_01_part2(&day1_file);
    println!("Day 1, part2: {}", result1_2);

    // DAY 2
    let mut day2_file = File::open("src/days/input_day2.txt").unwrap();
    let result2_1 = day_02::day_02_part1(&day2_file);
    println!("Day 2, part1: {}", result2_1);
    day2_file.seek(pos).unwrap();
    let result2_2 = day_02::day_02_part2(&day2_file);
    println!("Day 2, part2: {}", result2_2);

    // DAY 3
    let mut day3_file = File::open("src/days/input_day3.txt").unwrap();
    let result3_1 = day_03::day_03_part1(&day3_file);
    println!("Day 3, part1: {}", result3_1);
    day3_file.seek(pos).unwrap();
    let result3_2 = day_03::day_03_part2(&day3_file);
    println!("Day 3, part2: {}", result3_2);
}
