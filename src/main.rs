pub mod days;
mod utils;

fn main() -> () {
    println!("day 10 answer a:\n{}", days::day10::result_a().unwrap());
    println!("\nday 10 answer b:\n{}", days::day10::result_b().unwrap());
}
