#![allow(dead_code)]

use std::fs;
use std::str::FromStr;
use std::fmt::Debug;
use itertools::Itertools;


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn vector_of_two_string_tuples_from_file(file_path: &str) -> Vec<(String, String)> {
    /*
     * Assumes a file with two whitespace-separated strings on each line.
     */

    let file_contents: String = fs::read_to_string(file_path).unwrap();
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

fn str_to_i32(s: &str) -> i32
{
    if s.is_empty() { return 0 }
    return s.parse().unwrap()
}

pub fn file_name_to_i32_vec_allow_empty_lines(file_name: &str) -> Vec<i32>
{
    /*
     Takes as input a file name which contains a number per row.

     Returns an i32 vector of those numbers.
     */
    let file_contents = fs::read_to_string(file_name).unwrap();
    let numbers = file_contents
        .lines()
        .map(|s| str_to_i32(s))
        .collect();

    return numbers;
}

pub fn file_name_to_nr_vec<T>(file_name: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,  // The error type of T's Err must be
{                                // constrained. Needs to impl Debug
    /*
     Takes as input a file name which contains a number per row.

     Returns a vector of those numbers.

     Specify data type on function call, e.g.:
     let a: Vec<i32> = file_name_to_nr_vec(file_name);
     */
    let file_contents = fs::read_to_string(file_name).unwrap();
    let numbers: Vec<T> = file_contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    return numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_to_i32_vec_test() {
        let result: Vec<i32> = file_name_to_nr_vec("tests/inputs/numbers_on_rows.txt");
        assert_eq!(result, vec![1,2,3]);
    }

    #[test]
    fn file_name_to_f32_vec_test() {
        let result: Vec<f32> = file_name_to_nr_vec("tests/inputs/numbers_on_rows.txt");
        assert_eq!(result, vec![1.,2.,3.]);
    }
}
