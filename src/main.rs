pub mod days;
mod utils;

fn main() -> () {
    println!("day 15 answer a:\n{}", days::day15::result_a().unwrap());
    println!("\nday 15 answer b:\n{}", days::day15::result_b().unwrap());
}
