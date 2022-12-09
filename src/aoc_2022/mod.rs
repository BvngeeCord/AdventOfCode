use crate::Solver;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

pub fn solvers_2022() -> Vec<Box<dyn Solver>> {
    vec![
        Box::from(day_1::Day1{}),
        Box::from(day_2::Day2{}),
        Box::from(day_3::Day3{}),
        Box::from(day_4::Day4{}),
    ]
}
