pub mod days;
mod utils;

fn main() -> () {
    println!("day 8 answer a: {}", days::day8::result_a().unwrap());
    println!("day 8 answer b: {}", days::day8::result_b().unwrap());
}
