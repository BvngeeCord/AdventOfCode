use crate::{Solver, util};
use RPS::*;

pub struct Day2 {}

impl Solver for Day2 {
    fn solve(&mut self, part: u16) -> Result<i32, String> {

        let contents: String = util::load_from_text_file("./src/aoc_2022/day_2/input.txt")?;
        let lines: Vec<&str> = contents.lines().collect();

        let games: Vec<(RPS, RPS)> = lines.iter().map(move |string| -> (RPS, RPS) {
            match part {
                1 => {
                    let mut pair = string
                        .split(' ')
                        .map(move |char| -> RPS {
                            match char {
                                "A" | "X" => Rock, "B" | "Y" => Paper, "C" | "Z" => Scissors,
                                _ => Rock,
                            }
                        });
                    (pair.next().unwrap() , pair.next().unwrap())
                },
                2 => {
                    let mut chars = string.chars();
                    let opponent_play: RPS = match chars.next().unwrap() {
                        'A' => Rock, 'B' => Paper, 'C' => Scissors, _ => unreachable!()
                    };

                    (opponent_play, RPS::chose_move(chars.last().unwrap(), opponent_play))
                },
                _ => unreachable!()
            }
        }).collect();

        let mut score: u32 = 0;
        games.iter().for_each(|game| {
            score += game.1.battle(&game.0);
            score += match game.1 {
                Rock => 1, Paper => 2, Scissors => 3
            }
        });

        Ok(score as i32)
        
    }
}

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn battle(&self, other: &RPS) -> u32 {
        match self {
            Rock => { match other { Rock => 3, Paper => 0, Scissors => 6 } },
            Paper => { match other { Rock => 6, Paper => 3, Scissors => 0 } },
            Scissors => { match other { Rock => 0, Paper => 6, Scissors => 3 } }
        }
    }

    pub fn chose_move(outcome: char, other: RPS) -> RPS {
        match outcome {
            'X' => { match other { Rock => Scissors, Paper => Rock, Scissors => Paper} },
            'Y' => { other },
            'Z' => { match other { Rock => Paper, Paper => Scissors, Scissors => Rock } },
            _ => unreachable!()
        }
    }

}
