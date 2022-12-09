use std::ops::Range;

use crate::{Solver, util};

pub struct Day4 {}

impl Solver for Day4 {
    fn solve(&mut self, part: u16) -> Result<i32, String> {
        let contents = util::load_from_text_file(2022, 4).unwrap();

        let pairs: Vec<(Range<u32>, Range<u32>)> = contents.lines().map(|line| {
            let pair: Vec<Range<u32>> = line.split(',').map(|range| -> Range<u32> {
                let mut nums = range.split('-');
                let s1 = nums.next().unwrap();
                let s2 = nums.last().unwrap();
                str::parse::<u32>(s1).unwrap()..str::parse(s2).unwrap()
            }).collect();
            (pair.get(0).unwrap().clone(), pair.get(1).unwrap().clone())
        }).collect();

        let mut count = 0;
        for pair in pairs.iter() {
            let (r1, r2) = (pair.0.clone(), pair.1.clone());
            match part {
                1 => {
                    if (r1.start <= r2.start && r1.end >= r2.end) || (r2.start <= r1.start && r2.end >= r1.end) {
                        count += 1;
                    }
                },
                2 => {
                    if !(r1.start > r2.end || r2.start > r1.end) {
                        count += 1;
                    }
                },
                _ => unreachable!()
            };
        }

        Ok(count)
    }
}
