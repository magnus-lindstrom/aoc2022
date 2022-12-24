pub mod days;
mod utils;

fn main() -> () {
    println!("Part a:\n{}", days::day24::result_a().unwrap());
    println!("\nPart b:\n{}", days::day24::result_b().unwrap());
}
