use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

pub fn day_03_part1(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);
    let mut result: usize = 0;

    let mut characters_first_compartment: HashSet<char> = HashSet::new();
    let mut characters_second_compartment: HashSet<char> = HashSet::new();

    //Used later to calculate the actual score, these are both 1 character before A and a respectively
    let uppercase_comparator = '@' as usize;
    let lowercase_comparator = '`' as usize;

    for line in reader.lines() {
        let current_line = line.unwrap();

        if !current_line.is_empty() {
            let compartment_size = current_line.chars().count() / 2;
            //Inserts first compartment into the set
            current_line.chars().take(compartment_size).for_each(|c| {
                characters_first_compartment.insert(c);
            });

            //Inserts second half until one insert() returns false which indicates that it was already in the set
            current_line.chars().skip(compartment_size).for_each(|c| {
                characters_second_compartment.insert(c);
            });

            // It should only contain one character
            let common_char: char = characters_first_compartment
                .intersection(&characters_second_compartment)
                .next()
                .unwrap()
                .to_owned();
            // Extract unicode value of the character
            let numeric_char = common_char as usize;

            if common_char.is_lowercase() {
                result += numeric_char.abs_diff(lowercase_comparator);
            } else {
                result += numeric_char.abs_diff(uppercase_comparator) + 26;
            }
        }
        // Clear both sets for next iteration
        characters_first_compartment.clear();
        characters_second_compartment.clear();
    }

    result
}

pub fn day_03_part2(file: &std::fs::File) -> usize {
    let reader = BufReader::new(file);
    let mut result: usize = 0;

    //Used later to calculate the actual score, these are both 1 character before A and a respectively
    let uppercase_comparator = '@' as usize;
    let lowercase_comparator = '`' as usize;

    let mut lines = reader.lines();

    while let (Some(Ok(line1)), Some(Ok(line2)), Some(Ok(line3))) =
        (lines.next(), lines.next(), lines.next())
    {
        let character_sets: Vec<HashSet<char>> =
            vec![line1.chars().collect(), line2.chars().collect()];
        let mut s: HashSet<char> = line3.chars().collect();

        for set in character_sets {
            s = s.intersection(&set).copied().collect();
        }

        let common_char: char = s.iter().next().unwrap().to_owned();
        // Extract unicode value of the character
        let numeric_char = common_char as usize;

        if common_char.is_lowercase() {
            result += numeric_char.abs_diff(lowercase_comparator);
        } else {
            result += numeric_char.abs_diff(uppercase_comparator) + 26;
        }
    }
    result
}
