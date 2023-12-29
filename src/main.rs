#![warn(clippy::all, clippy::pedantic)]

pub trait Solution<T> {
    fn solution() -> T;
}

mod day_01 {
    pub mod part_01;
    pub mod part_02;
}

mod day_02 {
    pub mod part_01;
    pub mod part_02;
}

fn main() {
    // dbg!(day_01::part_01::solution("./src/day_01/input.txt"));
    // dbg!(day_01::part_02::solution("./src/day_01/input.txt"));

    // dbg!(day_02::part_01::Solution::solution());
    // dbg!(day_02::part_02::Solution::solution());
}
