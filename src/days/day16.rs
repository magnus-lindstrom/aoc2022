extern crate termion;

use std::io::{stdout, Write};
use termion::{color, style};

const _FILE_PATH: &str = "inputs/dayX.txt";

pub fn result_a() -> Result<i32, &'static str> {
    let mut stdout = stdout();
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic", style::Italic);

    print!(
        "{}{}hello",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    );
    stdout.flush().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    print!(
        "{}{}there",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    );
    std::thread::sleep(std::time::Duration::from_millis(1000));
    print!("{}{}foo", termion::clear::All, termion::cursor::Goto(1, 1));
    std::thread::sleep(std::time::Duration::from_millis(1000));
    print!("{}{}bar", termion::clear::All, termion::cursor::Goto(1, 1));
    Ok(0)
}

pub fn result_b() -> Result<i32, &'static str> {
    Ok(0)
}

/*
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn result_a_is_correct() {
let answer = result_a().unwrap();
assert_eq!(answer, 0);
}

#[test]
fn result_b_is_correct() {
let answer = result_b().unwrap();
assert_eq!(answer, 0);
}
}
*/
