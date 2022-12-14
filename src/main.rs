pub mod days;
mod utils;

fn main() -> () {
    println!("day 14 answer a:\n{}", days::day14::result_a().unwrap());
    println!("\nday 14 answer b:\n{}", days::day14::result_b().unwrap());
}
