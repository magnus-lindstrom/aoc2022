pub mod days;
mod utils;

fn main() -> () {
    println!("day 20 answer a:\n{}", days::day20::result_a().unwrap());
    println!("\nday 20 answer b:\n{}", days::day20::result_b().unwrap());
}
