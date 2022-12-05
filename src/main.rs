pub mod days;
mod utils;

fn main() -> () {
    println!("day 5 answer a: {}", days::day5::result_a().unwrap());
    println!("day 5 answer b: {}", days::day5::result_b().unwrap());
}
