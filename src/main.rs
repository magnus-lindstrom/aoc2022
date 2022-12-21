pub mod days;
mod utils;

fn main() -> () {
    println!("day 21 answer a:\n{}", days::day21::result_a().unwrap());
    println!("\nday 21 answer b:\n{}", days::day21::result_b().unwrap());
}
