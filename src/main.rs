#![allow(dead_code)]

mod utils;
mod days;


fn main() -> () {
    println!("day 3 answer a: {}", days::day3::result_a().unwrap());
    println!("day 3 answer b: {}", days::day3::result_b().unwrap());
}
