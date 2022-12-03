use crate::{Solver, util};
use RPS::*;

pub struct Day2 {

}

impl Solver for Day2 {
    fn solve(&mut self, part: u16) -> Result<i32, String> {

        let contents: String = util::load_from_text_file("./src/aoc_2022/day_2/input.txt")?;
        let lines: Vec<&str> = contents.lines().collect();

        let games: Vec<(RPS, RPS)> = lines.iter().map(move |string| -> (RPS, RPS) {
            let mut pair = string
                .split(' ')
                .map(move |char| -> RPS {
                    match char {
                        "A" | "X" => Rock, "B" | "Y" => Paper, "C" | "Z" => Scissors,
                        _ => Rock,
                    }
                });
            // println!("{} {}", string, iter.size_hint().1.unwrap());
            // (iter.next().unwrap(), iter.next().unwrap())
            (pair.next().unwrap() , pair.next().unwrap())
        }).collect();

        let mut score: u32 = 0;
        games.iter().for_each(|game| {
            score += game.1.battle(&game.0);
            score += match game.1 {
                Rock => 1, Paper => 2, Scissors => 3,
            }
        });

        Ok(score as i32)
        
    }
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    ///Returns an Outcome describing the outcome for this instance
    pub fn battle(&self, other: &RPS) -> u32 {
        match self {
            Rock => {
                match other { Rock => 3, Paper => 0, Scissors => 6 }
            },
            Paper => {
                match other { Rock => 6, Paper => 3, Scissors => 0 }
            },
            Scissors => {
                match other { Rock => 0, Paper => 6, Scissors => 3 }
            }
        }
    }
}
