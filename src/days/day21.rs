use crate::utils;
use std::collections::HashMap;
//const FILE_PATH: &str = "inputs/day21.txt";
const FILE_PATH: &str = "inputs/day21_test.txt";

struct Function {
    lhs: String,
    rhs: String,
    operation: Operation,
}
impl Function {
    fn new(v: Vec<String>) -> Function {
        let operation: Operation;
        if v[1] == "*" {
            operation = Operation::Multiplication;
        } else if v[1] == "+" {
            operation = Operation::Addition;
        } else if v[1] == "-" {
            operation = Operation::Subtraction;
        } else if v[1] == "/" {
            operation = Operation::Division;
        } else {
            panic!("unknown operation");
        }
        Function {
            lhs: v[0].clone(),
            rhs: v[2].clone(),
            operation,
        }
    }
}

#[derive(PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

fn find_out_monkey_nr(
    name: String,
    monkey_to_nr: &mut HashMap<String, i64>,
    monkey_to_function: &HashMap<String, Function>,
) -> Option<i64> {
    if monkey_to_nr.contains_key(&name) {
        return Some(monkey_to_nr[&name]);
    }
    let operation = &monkey_to_function[&name].operation;
    let lhs_nr = find_out_monkey_nr(
        monkey_to_function[&name].lhs.clone(),
        monkey_to_nr,
        monkey_to_function,
    )
    .unwrap();
    let rhs_nr = find_out_monkey_nr(
        monkey_to_function[&name].rhs.clone(),
        monkey_to_nr,
        monkey_to_function,
    )
    .unwrap();
    let result: i64;
    if operation == &Operation::Addition {
        result = lhs_nr + rhs_nr;
    } else if operation == &Operation::Subtraction {
        result = lhs_nr - rhs_nr;
    } else if operation == &Operation::Multiplication {
        result = lhs_nr * rhs_nr;
    } else {
        result = lhs_nr / rhs_nr;
    }
    monkey_to_nr.insert(name, result);
    Some(result)
}

fn what_to_yell(
    name: String,
    monkey_to_nr: &mut HashMap<String, i64>,
    monkey_to_function: &HashMap<String, Function>,
) -> i64 {
    if monkey_to_nr.contains_key(&name) {
        return monkey_to_nr[&name];
    }
    let operation = &monkey_to_function[&name].operation;
    let lhs_nr = find_out_monkey_nr(
        monkey_to_function[&name].lhs.clone(),
        monkey_to_nr,
        monkey_to_function,
    );
    let rhs_nr = find_out_monkey_nr(
        monkey_to_function[&name].rhs.clone(),
        monkey_to_nr,
        monkey_to_function,
    );
    let result: i64 = 0;
    /*
    if operation == &Operation::Addition {
    result = lhs_nr + rhs_nr;
    } else if operation == &Operation::Subtraction {
    result = lhs_nr - rhs_nr;
    } else if operation == &Operation::Multiplication {
    result = lhs_nr * rhs_nr;
    } else {
    result = lhs_nr / rhs_nr;
    }
    monkey_to_nr.insert(name, result);
    */
    result
}

pub fn result_a() -> Result<i64, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    let mut monkey_to_nr: HashMap<String, i64> = HashMap::new();
    let mut monkey_to_function: HashMap<String, Function> = HashMap::new();
    for line in input.iter() {
        let monkey_name: &str = line[0].strip_suffix(':').unwrap();
        let potential_nr: Result<i64, _> = line[1].parse::<i64>();
        if let Ok(nr) = potential_nr {
            monkey_to_nr.insert(monkey_name.to_string(), nr);
        } else {
            monkey_to_function.insert(monkey_name.to_string(), Function::new(line[1..].to_vec()));
        }
    }
    Ok(find_out_monkey_nr("root".to_string(), &mut monkey_to_nr, &monkey_to_function).unwrap())
}

pub fn result_b() -> Result<i64, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    let mut monkey_to_nr: HashMap<String, i64> = HashMap::new();
    let mut monkey_to_function: HashMap<String, Function> = HashMap::new();
    for line in input.iter() {
        let monkey_name: &str = line[0].strip_suffix(':').unwrap();
        let potential_nr: Result<i64, _> = line[1].parse::<i64>();
        if let Ok(nr) = potential_nr {
            monkey_to_nr.insert(monkey_name.to_string(), nr);
        } else {
            monkey_to_function.insert(monkey_name.to_string(), Function::new(line[1..].to_vec()));
        }
    }

    return Ok(0);
    /*
    let nr_of_monkey: Option<i64> = find_out_monkey_nr
        Ok(find_out_monkey_nr(
                "root".to_string(),
                &mut monkey_to_nr,
                &monkey_to_function,
                ))
                */
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
