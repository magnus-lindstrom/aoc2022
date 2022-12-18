pub mod days;
mod utils;

fn main() -> () {
    println!("day 18 answer a:\n{}", days::day18::result_a().unwrap());
    println!("\nday 18 answer b:\n{}", days::day18::result_b().unwrap());
}
