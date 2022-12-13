use crate::utils;
use std::cmp::Ordering;
const FILE_PATH: &str = "inputs/day13.txt";
//const FILE_PATH: &str = "inputs/day13_test.txt";
//const FILE_PATH: &str = "inputs/day13_scratch.txt";
//const FILE_PATH: &str = "inputs/day13_viktor.txt";

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
                    i = j;
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

impl Eq for Object {}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Object {
    fn cmp(self: &Object, other: &Self) -> Ordering {
        for i in 0..other.numbers.len() {
            if i >= self.numbers.len() {
                return Ordering::Less;
            }
            let self_is_integer: bool = self.objects[i].is_none();
            let other_is_integer: bool = other.objects[i].is_none();
            if self_is_integer && other_is_integer {
                if self.numbers[i] < other.numbers[i] {
                    return Ordering::Less;
                } else if self.numbers[i] > other.numbers[i] {
                    return Ordering::Greater;
                }
            } else if !self_is_integer && !other_is_integer {
                let left_copy: &Object = self.objects[i].as_ref().unwrap();
                let right_copy: &Object = other.objects[i].as_ref().unwrap();
                let new_objects_in_right_order = left_copy.cmp(right_copy);
                if new_objects_in_right_order == Ordering::Less {
                    return Ordering::Less;
                } else if new_objects_in_right_order == Ordering::Greater {
                    return Ordering::Greater;
                }
            } else if self_is_integer && !other_is_integer {
                let nr: i32 = self.numbers[i].unwrap();
                let new_self_obj: Object = Object {
                    numbers: vec![Some(nr)],
                    objects: vec![None],
                };
                let other_copy: &Object = other.objects[i].as_ref().unwrap();
                let new_objects_order = new_self_obj.cmp(other_copy);
                if new_objects_order == Ordering::Less {
                    return Ordering::Less;
                } else if new_objects_order == Ordering::Greater {
                    return Ordering::Greater;
                }
            } else if !self_is_integer && other_is_integer {
                let nr: i32 = other.numbers[i].unwrap();
                let new_other_obj: Object = Object {
                    numbers: vec![Some(nr)],
                    objects: vec![None],
                };
                let self_copy: &Object = self.objects[i].as_ref().unwrap();
                let new_objects_order = self_copy.cmp(&new_other_obj);
                if new_objects_order == Ordering::Less {
                    return Ordering::Less;
                } else if new_objects_order == Ordering::Greater {
                    return Ordering::Greater;
                }
            } else {
                panic!("{}", format!("panic at i = {}", i));
            }
        }
        if other.numbers.len() < self.numbers.len() {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
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
        let order = object_pairs[i].0.cmp(&object_pairs[i].1);
        if order == Ordering::Less {
            sum_of_correct_indeces += i as i32 + 1;
        } else if order == Ordering::Equal {
            panic!("Top objects should not be equal!");
        }
    }

    Ok(sum_of_correct_indeces)
}

pub fn result_b() -> Result<i32, &'static str> {
    let input = utils::file_path_to_vec_of_vec_of_lines_separated_by_blanks(FILE_PATH);
    let mut objects: Vec<Object> = Vec::new();
    for i in 0..input.len() {
        let left_object: Object = Object::new(input[i][0].clone());
        let right_object: Object = Object::new(input[i][1].clone());
        objects.push(right_object);
        objects.push(left_object);
    }
    let two_object = Object::new("[[2]]".to_string());
    let two_object_copy = Object::new("[[2]]".to_string());
    let six_object = Object::new("[[6]]".to_string());
    let six_object_copy = Object::new("[[6]]".to_string());
    objects.push(two_object);
    objects.push(six_object);

    objects.sort();

    let mut prod_of_indeces: i32 = 1;
    for (i, object) in objects.iter().enumerate() {
        if object == &two_object_copy || object == &six_object_copy {
            prod_of_indeces *= i as i32 + 1;
        }
    }

    Ok(prod_of_indeces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 6072);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 22184);
    }
}
