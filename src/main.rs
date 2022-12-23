pub mod days;
mod utils;

fn main() -> () {
    println!("Part a:\n{}", days::day23::result_a().unwrap());
    println!("\nPart b:\n{}", days::day23::result_b().unwrap());
}
