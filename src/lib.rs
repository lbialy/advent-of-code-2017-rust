pub trait Puzzle {
    fn name(&self) -> String;
    fn solution(&self) -> String;
}