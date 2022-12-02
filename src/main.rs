mod util;
mod args;
mod aoc_2022;

use std::collections::HashMap;

use args::ProgramArgs;
use clap::Parser;

pub trait Solver {
    fn solve(&mut self) -> i32;
}

fn main() -> Result<(), String> {
    let args = ProgramArgs::parse();
    let (year, day) = (args.year, args.day);

    if year < 2022 {
        return Err("Invalid year!".to_string())
    }

    if day == 0 || day > 31 {
        return Err("Invalid day!".to_string())
    }

    let mut solvers = solvers_mapped_to_year();

    if let Some(days) = solvers.get_mut(&year) {

        if let Some(solver) = days.get_mut(day as usize - 1) {
            println!("Year: {}, Day: {}, Output: {}", year, day, solver.solve());
        } else {
            return Err("Error finding day's solver!".to_string())
        }

    } else {
        return Err("Error finding year's list of solvers!".to_string())
    }

    Ok(())
}

fn solvers_mapped_to_year() -> HashMap<u16, Vec<Box<dyn Solver>>> {
    let mut map = HashMap::new();
    map.insert(2022, aoc_2022::solvers_2022());

    map
}