#![allow(dead_code)]

use std::fs;
use std::str::FromStr;
use std::fmt::Debug;
use itertools::Itertools;


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn vector_of_strings_from_file(file_path: &str) -> Vec<String> {

    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());

    let mut output: Vec<String> = Vec::new();
    for line in file_contents.lines() {

        output.push(line.to_string());

    }
    return output;

}

pub fn vector_of_two_string_tuples_from_file(file_path: &str) -> Vec<(String, String)> {
    /*
     * Assumes a file with two whitespace-separated strings on each line.
     */

    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut a: String;
    let mut b: String;
    let mut a_str: &str;
    let mut b_str: &str;

    let mut output: Vec<(String, String)> = Vec::new();

    for line in file_contents.lines() {

        (a_str, b_str) = line.split_whitespace().collect_tuple().unwrap();
        a = a_str.to_string();
        b = b_str.to_string();
        output.push((a, b));
    }
    return output;

    /*
    let output: Vec<(String, String)> = file_contents
        .lines()
        .map(|s| s
             .split_whitespace()
             .collect_tuple()  // collects a maximum of 12 elements into a tuple
             .unwrap()
        ).collect();
    return output;
    */
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

pub fn file_path_to_nr_vec_allow_empty_lines<T>(file_path: &str) -> Vec<T>
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
