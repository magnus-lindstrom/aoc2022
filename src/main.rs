pub mod days;
mod utils;

fn main() -> () {
    println!("day 4 answer a: {}", days::day4::result_a().unwrap());
    println!("day 4 answer b: {}", days::day4::result_b().unwrap());
}
