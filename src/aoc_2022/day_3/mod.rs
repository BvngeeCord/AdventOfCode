use std::str::Lines;

use crate::{Solver, util};

pub struct Day3 {}

impl Solver for Day3 {
    fn solve(&mut self, part: u16) -> Result<i32, String> {
        let contents = util::load_from_text_file("./src/aoc_2022/day_3/input.txt").unwrap();
        let rucksacks: Lines = contents.lines();

        let unique_chars = match part {
            1 => Day3::process_rucksacks_part1(rucksacks),
            2 => Day3::process_rucksacks_part2(rucksacks),
            _ => unreachable!()
        };

        let mut sum_of_priorities = 0;
        for c in unique_chars {
            sum_of_priorities += {
                if c.is_ascii_lowercase() {
                    c as u32 - 96
                } else {
                    c as u32 - 38
                }
            };
        }

        Ok(sum_of_priorities as i32)
    }

}

impl Day3 {
    pub fn to_char_set(str: &str) -> Vec<char> {
        let mut chars = str.chars().collect::<Vec<char>>();
        chars.sort();
        chars.dedup();

        chars
    }

    ///returns a list of the unique characters - part 1 logic
    pub fn process_rucksacks_part1(rucksacks: Lines) -> Vec<char> {
        let mut unique_chars = Vec::new();
        for rucksack in rucksacks {
            let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
            let (set1, set2) = (Day3::to_char_set(compartment_1), Day3::to_char_set(compartment_2));

            let mut double_char: char = ' ';
            for c in set1 {
                if set2.contains(&c) {
                    double_char = c;
                    break;
                }
            }
            unique_chars.push(double_char);
        }

        unique_chars
    }
    
    ///returns a list of the unique characters - part 2 logic
    pub fn process_rucksacks_part2(mut rucksacks: Lines) -> Vec<char> {
        let mut unique_chars = Vec::new();

        while let (Some(r1), Some(r2), Some(r3)) = (rucksacks.next(), rucksacks.next(), rucksacks.next()) {
            for c in r1.chars() {
                if r2.contains(c) && r3.contains(c) {
                    unique_chars.push(c);
                    break;
                }
            }
        }

        println!("{:?}", unique_chars);
        unique_chars
    }
}
