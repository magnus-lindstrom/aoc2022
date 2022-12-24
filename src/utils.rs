extern crate termion;

use std::io::{stdout, Write};

use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn draw_and_sleep_ms(string: &str, sleep_time: u64) -> () {
    let mut stdout = stdout();
    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        string
    );
    stdout.flush().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(sleep_time));
}

fn str_to_nr<T>(s: &str) -> T
where
    T: FromStr,
    // The error type of T's Err must be constrained. Needs to impl Debug
    <T as FromStr>::Err: Debug,
{
    if s.is_empty() {
        return "0".parse().unwrap();
    }
    s.parse().unwrap()
}

pub fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}


/// assumes that lines come in pairs, separated by a blank line
pub fn file_path_to_vec_of_vec_of_lines_separated_by_blanks(file_path: &str) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    let mut inner_vec: Vec<String> = Vec::new();
    for line in std::fs::read_to_string(file_path).unwrap().lines() {
        if line == "" {
            output.push(inner_vec);
            inner_vec = Vec::new();
        } else {
            inner_vec.push(line.to_string());
        }
    }
    output.push(inner_vec); // also add last two lines (assuming file does not end with blank line)
    output
}

pub fn file_path_to_vec_of_char_vecs(file_path: &str) -> Vec<Vec<char>> {
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut output: Vec<Vec<char>> = Vec::new();
    for line in file_contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        output.push(row);
    }
    output
}

pub fn file_path_to_nr_matrix<T>(file_path: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    // The error type of T's Err must be constrained. Needs to impl Debug
    <T as FromStr>::Err: Debug,
{
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut output: Vec<Vec<T>> = Vec::new();
    for line in file_contents.lines() {
        let mut row: Vec<T> = Vec::new();
        for ch in line.chars() {
            row.push(str_to_nr(&ch.to_string()));
        }
        output.push(row);
    }
    output
}

pub fn file_path_plus_inner_sep_plus_outer_sep_to_vec_of_vec_of_vec_of_nr<T>(
    file_path: &str,
    inner_sep: &str,
    outer_sep: &str,
) -> Vec<Vec<Vec<T>>>
where
    T: FromStr,
    // The error type of T's Err must be constrained. Needs to impl Debug
    <T as FromStr>::Err: Debug,
{
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut outer_vec: Vec<Vec<Vec<T>>> = Vec::new();

    for line in file_contents.lines() {
        let mut middle_vec: Vec<Vec<T>> = Vec::new();
        let after_first_split: Vec<&str> = line.split(outer_sep).collect();
        for substring in after_first_split {
            let after_second_split = substring.split(inner_sep);
            let mut inner_vec: Vec<T> = Vec::new();
            for subsubstring in after_second_split {
                let nr: T = str_to_nr(subsubstring);
                inner_vec.push(nr);
            }
            middle_vec.push(inner_vec);
        }
        outer_vec.push(middle_vec);
    }
    outer_vec
}

pub fn file_path_to_vec_of_char_nr_tuples<T>(file_path: &str) -> Vec<(char, T)>
where
    T: FromStr,
    // The error type of T's Err must be constrained. Needs to impl Debug
    <T as FromStr>::Err: Debug,
{
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut output: Vec<(char, T)> = Vec::new();
    for line in file_contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let ch_vec: Vec<char> = words[0].chars().collect();
        assert_eq!(ch_vec.len(), 1);
        let ch: char = ch_vec[0];
        let nr: T = str_to_nr(words[1]);
        output.push((ch, nr));
    }
    output
}

pub fn file_path_to_vec_of_strings_preserve_whitespace(file_path: &str) -> Vec<String> {
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());

    let mut output: Vec<String> = Vec::new();

    for line in file_contents.lines() {
        output.push(line.to_string());
    }
    return output;
}

pub fn vector_of_string_vectors_from_file(file_path: &str) -> Vec<Vec<String>> {
    /*
     * Assumes a file with whitespace-separated strings on each line.
     */

    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());

    let mut output: Vec<Vec<String>> = Vec::new();
    let mut inner_vector: Vec<String>;

    for line in file_contents.lines() {
        inner_vector = Vec::new();
        for substring in line.split_whitespace() {
            inner_vector.push(substring.to_string());
        }
        output.push(inner_vector);
    }
    return output;
}

pub fn nr_vec_from_file_allow_empty_lines<T>(file_path: &str) -> Vec<T>
where
    T: FromStr,
    // The error type of T's Err must be constrained. Needs to impl Debug
    <T as FromStr>::Err: Debug,
{
    /*
    Takes as input a file name which contains a number per row.

    Empty lines will be interpreted as a zero.

    Returns a vector of those numbers.

    Specify data type on function call, e.g.:
    let a: Vec<i32> = file_path_to_nr_vec(file_path);
    */
    let file_contents = fs::read_to_string(file_path).unwrap();
    let numbers: Vec<T> = file_contents.lines().map(|s| str_to_nr(s)).collect();

    return numbers;
}
