use crate::utils;
//const FILE_PATH: &str = "inputs/day13.txt";
const FILE_PATH: &str = "inputs/day13_test.txt";

#[derive(Debug)]
struct Object {
    numbers: Vec<Option<i32>>,
    objects: Vec<Option<Object>>,
}
impl Object {
    fn new(string: String) -> Object {
        let mut chars = string.chars();
        chars.next(); // remove leading [
        chars.next_back(); // remove trailing ]
        let new_string: String = chars.collect();

        let mut my_numbers: Vec<Option<i32>> = Vec::new();
        let mut my_objects: Vec<Option<Object>> = Vec::new();

        let mut i: usize = 0;
        while i < new_string.len() {
            let ch_i = new_string.chars().nth(i).unwrap();
            if ch_i == '[' {
                // find matching ]
                let substring_start: usize = i;
                let mut substring_end: Option<usize> = None;
                let mut nr_of_closing_brackets_needed = 1;
                for j in i + 1..new_string.len() {
                    let ch_j = new_string.chars().nth(j).unwrap();
                    if ch_j == '[' {
                        nr_of_closing_brackets_needed += 1;
                    } else if ch_j == ']' {
                        nr_of_closing_brackets_needed -= 1;
                    }
                    if nr_of_closing_brackets_needed == 0 {
                        substring_end = Some(j);
                        break;
                    }
                }
                let substring: String = new_string
                    [substring_start..=substring_end.expect("Did not find end of substring")]
                    .to_string();
                my_objects.push(Some(Object::new(substring)));
                my_numbers.push(None);
                i = substring_end.unwrap();
            } else if ch_i == ',' {
            } else {
                let substring_start: usize = i;
                let mut substring_end: usize = i; // to be updated if needed
                for j in i + 1..new_string.len() {
                    let ch_j = new_string.chars().nth(j).unwrap();
                    if vec![',', '[', ']'].contains(&ch_j) {
                        break;
                    }
                    substring_end = j;
                }
                let substring: String = new_string[substring_start..=substring_end].to_string();
                let nr_from_substring: i32 = substring.parse().unwrap();
                my_objects.push(None);
                my_numbers.push(Some(nr_from_substring));
            }
            i += 1;
        }

        Object {
            numbers: my_numbers,
            objects: my_objects,
        }
    }
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::file_path_to_vec_of_vec_of_lines_separated_by_blanks(FILE_PATH);
    println!("{:?}", input);
    for i in 0..input.len() {
        let left_object: Object = Object::new(input[i][0].clone());
        let right_object: Object = Object::new(input[i][1].clone());
        println!("{:?}", left_object);
        println!("{:?}\n", right_object);
    }
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
