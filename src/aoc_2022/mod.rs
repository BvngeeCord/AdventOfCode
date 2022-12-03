use crate::Solver;

pub mod day_1;
pub mod day_2;

pub fn solvers_2022() -> Vec<Box<dyn Solver>> {
    vec![
        Box::from(day_1::Day1{}),
        Box::from(day_2::Day2{})
    ]
}
