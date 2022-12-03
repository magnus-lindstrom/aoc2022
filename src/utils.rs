#![allow(dead_code)]

use std::fs;
use std::str::FromStr;
use std::fmt::Debug;


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn str_to_nr<T>(s: &str) -> T
where
    T: FromStr,
    // The error type of T's Err must be constrained. Needs to impl Debug
    <T as FromStr>::Err: Debug,
{
    if s.is_empty() { return "0".parse().unwrap() }
    s.parse().unwrap()
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
    let numbers: Vec<T> = file_contents
        .lines()
        .map(|s| str_to_nr(s))
        .collect();

    return numbers;
}
