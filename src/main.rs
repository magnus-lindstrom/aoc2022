pub mod days;
mod utils;

fn main() -> () {
    println!("day 11 answer a:\n{}", days::day11::result_a().unwrap());
    println!(
        "\nday 11 answer b:\n{}",
        days::day11::result_b(true).unwrap()
    );
}
