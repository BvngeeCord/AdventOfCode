use clap:: Parser;

#[derive(Debug, Parser)]
#[clap(author, about)]
pub struct ProgramArgs {

    ///The year of Advent of Code to run challenges from
    pub year: u16,
    
    ///The day of the month to run the challenge for
    pub day: u16,

    ///The question part to solve for
    pub part: u16,

}
