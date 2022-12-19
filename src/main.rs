pub mod days;
mod utils;

fn main() -> () {
    println!("day 19 answer a:\n{}", days::day19::result_a().unwrap());
    println!("\nday 19 answer b:\n{}", days::day19::result_b().unwrap());
}
