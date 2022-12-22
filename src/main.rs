pub mod days;
mod utils;

fn main() -> () {
    println!("Part a:\n{}", days::day16::result_a().unwrap());
    println!("\nPart b:\n{}", days::day16::result_b().unwrap());
}
