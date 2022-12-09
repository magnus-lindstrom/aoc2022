pub mod days;
mod utils;

fn main() -> () {
    println!("day 9 answer a: {}", days::day9::result_a().unwrap());
    println!("day 9 answer b: {}", days::day9::result_b().unwrap());
}
