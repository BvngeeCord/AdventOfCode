mod util;
mod args;
mod aoc_2022;

use std::collections::HashMap;

use args::ProgramArgs;
use clap::Parser;

pub trait Solver {
    fn solve(&mut self, part: u16) -> Result<i32, String>;
}

fn main() -> Result<(), String> {
    let args = ProgramArgs::parse();
    let (year, day, part) = (args.year, args.day, args.part);

    if year < 2022 {
        return Err("Invalid year!".to_string())
    }

    if day == 0 || day > 31 {
        return Err("Invalid day!".to_string())
    }

    let mut solvers = solvers_mapped_to_year();

    if let Some(days) = solvers.get_mut(&year) {

        if let Some(solver) = days.get_mut(day as usize - 1) {
            println!("Year: {}, Day: {}, Part: {} Output: {}", year, day, part, solver.solve(part)?);
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
