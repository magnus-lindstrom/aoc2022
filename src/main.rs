pub mod days;
mod utils;

fn main() -> () {
    println!("day 13 answer a:\n{}", days::day13::result_a().unwrap());
    println!("\nday 13 answer b:\n{}", days::day13::result_b().unwrap());
}
