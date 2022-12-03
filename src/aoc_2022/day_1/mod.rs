use crate::{Solver, util};

pub struct Day1 {
    //blabla
}

impl Solver for Day1 {
    fn solve(&mut self, part: u16) -> Result<i32, String> {
        
        let contents: String = util::load_from_text_file("./src/aoc_2022/day_1/input.txt")?;
        let lines: Vec<&str> = contents.lines().collect();
        
        let mut elves: Vec<i32> = Vec::new();
        
        lines.split(|string| string.is_empty()).for_each(|slice| {
            elves.push(
                slice.iter().map(|line| -> i32 {line.parse().unwrap()}).sum()
            )
        });

        match part {
            1 => {
                Ok(elves.iter().max().unwrap().to_owned())
            },
            2 => {
                //TODO: better method than dumb sorting and taking first three????
                // let mut iter = elves.iter();
                // let mut result: [i32; 3] = iter.take(3).map(|n| n.to_owned()).try_into().unwrap();
                elves.sort();
                elves.reverse();
                
                Ok(elves.iter().take(3).sum())
            },
            _ => {
                Err("Invalid part number (or part not complete yet)!".to_string())
            }
        }


    }
}
