mod lib;
mod day1;

use lib::Puzzle;
use day1::Day1A;
use day1::Day1B;

fn main() {
    let puzzles: Vec<Box<Puzzle>> = vec![
        Box::new(Day1A {}), Box::new(Day1B {})
    ];

    puzzles.into_iter().for_each(|puzzle| {
        println!("{} - {}", puzzle.name(), puzzle.solution());
    });
}

