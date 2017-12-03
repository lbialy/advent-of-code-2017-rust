mod lib;
mod day1;
mod day2;

use lib::Puzzle;
use day1::*;
use day2::*;

fn main() {
    let puzzles: Vec<Box<Puzzle>> = vec![
        Box::new(Day1A {}), Box::new(Day1B {}),
        Box::new(Day2A {}), Box::new(Day2B {})
    ];

    puzzles.into_iter().for_each(|puzzle| {
        println!("{} - {}", puzzle.name(), puzzle.solution());
    });
}

