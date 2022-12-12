pub mod days;
mod utils;

fn main() -> () {
    println!("day 12 answer a:\n{}", days::day12::result_a().unwrap());
    println!("\nday 12 answer b:\n{}", days::day12::result_b().unwrap());
}
