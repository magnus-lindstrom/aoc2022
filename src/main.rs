pub mod days;
mod utils;

fn main() -> () {
    println!("day 6 answer a: {}", days::day6::result_a().unwrap());
    println!("day 6 answer b: {}", days::day6::result_b().unwrap());
}
