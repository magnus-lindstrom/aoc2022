pub mod days;
mod utils;

fn main() -> () {
    println!("day 7 answer a: {}", days::day7::result_a().unwrap());
    println!("day 7 answer b: {}", days::day7::result_b().unwrap());
}
