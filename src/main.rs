pub mod days;
mod utils;

fn main() -> () {
    println!("day 17 answer a:\n{}", days::day17::result_a().unwrap());
    println!("\nday 17 answer b:\n{}", days::day17::result_b().unwrap());
}
