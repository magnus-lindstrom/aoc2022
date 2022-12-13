use crate::utils;
const FILE_PATH: &str = "inputs/day13.txt";
//const FILE_PATH: &str = "inputs/day13_test.txt";
//const FILE_PATH: &str = "inputs/day13_scratch.txt";

#[derive(Debug, Clone)]
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

fn objects_in_right_order(left_object: &Object, right_object: &Object, depth: i32) -> Option<bool> {
    println!("depth: {}", depth);
    // println!(
    // "receiving objects {:?}\nand\n{:?}",
    // left_object, right_object
    // );
    for i in 0..right_object.numbers.len() {
        if i >= left_object.numbers.len() {
            println!("true because left list ran out of elements first");
            return Some(true);
        }
        let left_is_integer: bool = left_object.objects[i].is_none();
        let right_is_integer: bool = right_object.objects[i].is_none();
        if left_is_integer && right_is_integer {
            if left_object.numbers[i] < right_object.numbers[i] {
                println!(
                    "true because {} < {}",
                    left_object.numbers[i].unwrap(),
                    right_object.numbers[i].unwrap()
                );
                return Some(true);
            } else if left_object.numbers[i] > right_object.numbers[i] {
                println!(
                    "false because {} > {}",
                    left_object.numbers[i].unwrap(),
                    right_object.numbers[i].unwrap()
                );
                return Some(false);
            }
            println!(
                "None because {} == {}",
                left_object.numbers[i].unwrap(),
                right_object.numbers[i].unwrap()
            );
        } else if !left_is_integer && !right_is_integer {
            let left_copy: &Object = left_object.objects[i].as_ref().unwrap();
            let right_copy: &Object = right_object.objects[i].as_ref().unwrap();
            println!("both elements are objects, go deeper");
            let new_objects_in_right_order =
                objects_in_right_order(left_copy, right_copy, depth + 1);
            if new_objects_in_right_order == Some(true) {
                return Some(true);
            } else if new_objects_in_right_order == Some(false) {
                return Some(false);
            }
        } else if left_is_integer && !right_is_integer {
            println!(
                "converting left ({}) to object and going deeper",
                left_object.numbers[i].unwrap()
            );
            let nr: i32 = left_object.numbers[i].unwrap();
            let new_left_obj: Object = Object {
                numbers: vec![Some(nr)],
                objects: vec![None],
            };
            let right_copy: &Object = right_object.objects[i].as_ref().unwrap();
            let new_objects_in_right_order =
                objects_in_right_order(&new_left_obj, right_copy, depth + 1);
            if new_objects_in_right_order == Some(true) {
                return Some(true);
            } else if new_objects_in_right_order == Some(false) {
                return Some(false);
            }
        } else if !left_is_integer && right_is_integer {
            println!(
                "converting right ({}) to object and going deeper",
                right_object.numbers[i].unwrap()
            );
            let nr: i32 = right_object.numbers[i].unwrap();
            let new_right_obj: Object = Object {
                numbers: vec![Some(nr)],
                objects: vec![None],
            };
            let left_copy: &Object = left_object.objects[i].as_ref().unwrap();
            let new_objects_in_right_order =
                objects_in_right_order(left_copy, &new_right_obj, depth + 1);
            if new_objects_in_right_order == Some(true) {
                return Some(true);
            } else if new_objects_in_right_order == Some(false) {
                return Some(false);
            }
        } else {
            panic!("{}", format!("panic at i = {}", i));
        }
    }
    if right_object.numbers.len() < left_object.numbers.len() {
        println!("false because right list ran out of elements first");
        return Some(false);
    }
    None
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::file_path_to_vec_of_vec_of_lines_separated_by_blanks(FILE_PATH);
    let mut object_pairs: Vec<(Object, Object)> = Vec::new();
    let mut sum_of_correct_indeces: i32 = 0;
    for i in 0..input.len() {
        let left_object: Object = Object::new(input[i][0].clone());
        let right_object: Object = Object::new(input[i][1].clone());
        object_pairs.push((left_object, right_object));
    }
    for i in 0..object_pairs.len() {
        let right_order = objects_in_right_order(&object_pairs[i].0, &object_pairs[i].1, 0);
        if right_order == Some(true) {
            println!("pair {} is in right order\n", i + 1);
            sum_of_correct_indeces += i as i32 + 1;
        } else if right_order == Some(false) {
            println!("pair {} is in the wrong order\n", i + 1);
        } else if right_order == None {
            println!("left object: {:?}", object_pairs[i].0);
            println!("right object: {:?}", object_pairs[i].1);
            panic!("Top objects should not be equal!");
        }
    }

    Ok(sum_of_correct_indeces)
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
