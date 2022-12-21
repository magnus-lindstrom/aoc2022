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

fn get_lhs_nr(nr_to_produce: i64, rhs: i64, operation: &Operation) -> i64 {
    let nr: i64;
    if operation == &Operation::Addition {
        nr = nr_to_produce - rhs;
    } else if operation == &Operation::Subtraction {
        nr = nr_to_produce + rhs;
    } else if operation == &Operation::Multiplication {
        nr = nr_to_produce / rhs;
    } else {
        nr = nr_to_produce * rhs;
    }
    nr
}

fn get_rhs_nr(nr_to_produce: i64, lhs: i64, operation: &Operation) -> i64 {
    let nr: i64;
    if operation == &Operation::Addition {
        nr = nr_to_produce - lhs;
    } else if operation == &Operation::Subtraction {
        nr = lhs - nr_to_produce;
    } else if operation == &Operation::Multiplication {
        nr = nr_to_produce / lhs;
    } else {
        nr = lhs / nr_to_produce;
    }
    nr
}

// if nr_to_produce is None, and one of the monkeys is me, returns None
//
// otherwise, returns what I should yell
fn what_should_i_yell(
    name: String,
    monkey_to_nr: &mut HashMap<String, i64>,
    monkey_to_function: &HashMap<String, Function>,
    nr_to_produce: Option<i64>,
) -> Option<i64> {
    if &name != "humn" && monkey_to_nr.contains_key(&name) {
        return Some(monkey_to_nr[&name]);
    }

    let operation = &monkey_to_function[&name].operation;
    let lhs_monkey = monkey_to_function[&name].lhs.clone();
    let rhs_monkey = monkey_to_function[&name].rhs.clone();

    if nr_to_produce == None {
        if lhs_monkey == "humn".to_string() || rhs_monkey == "humn".to_string() {
            println!("found \"humn\"");
            return None;
        }

        let lhs_nr = what_should_i_yell(lhs_monkey.clone(), monkey_to_nr, monkey_to_function, None);
        let rhs_nr = what_should_i_yell(rhs_monkey.clone(), monkey_to_nr, monkey_to_function, None);

        if lhs_nr == None || rhs_nr == None {
            return None;
        }

        let result: i64;
        if operation == &Operation::Addition {
            result = lhs_nr.unwrap() + rhs_nr.unwrap();
        } else if operation == &Operation::Subtraction {
            result = lhs_nr.unwrap() - rhs_nr.unwrap();
        } else if operation == &Operation::Multiplication {
            result = lhs_nr.unwrap() * rhs_nr.unwrap();
        } else {
            result = lhs_nr.unwrap() / rhs_nr.unwrap();
        }
        monkey_to_nr.insert(name, result);
        return Some(result);
    } else {
        if lhs_monkey == "humn".to_string() {
            let rhs_nr =
                what_should_i_yell(rhs_monkey.clone(), monkey_to_nr, monkey_to_function, None)
                    .unwrap();
            let to_yell: i64 = get_lhs_nr(nr_to_produce.unwrap(), rhs_nr, operation);
            println!("lhs is human, i should yell {}", to_yell);
            return Some(to_yell);
        } else if rhs_monkey == "humn".to_string() {
            let lhs_nr =
                what_should_i_yell(lhs_monkey.clone(), monkey_to_nr, monkey_to_function, None)
                    .unwrap();
            let to_yell: i64 = get_rhs_nr(nr_to_produce.unwrap(), lhs_nr, operation);
            println!("rhs is human, i should yell {}", to_yell);
        }

        let lhs_nr = what_should_i_yell(lhs_monkey.clone(), monkey_to_nr, monkey_to_function, None);
        let rhs_nr = what_should_i_yell(rhs_monkey.clone(), monkey_to_nr, monkey_to_function, None);

        if lhs_nr == None {
            if rhs_nr == None {
                panic!("both should not be None");
            }

            let new_nr_to_produce: i64 =
                get_lhs_nr(nr_to_produce.unwrap(), rhs_nr.unwrap(), operation);

            return what_should_i_yell(
                rhs_monkey.clone(),
                monkey_to_nr,
                monkey_to_function,
                Some(new_nr_to_produce),
            );
        } else if rhs_nr == None {
            if lhs_nr == None {
                panic!("both should not be None");
            }

            let new_nr_to_produce: i64 =
                get_rhs_nr(nr_to_produce.unwrap(), lhs_nr.unwrap(), operation);

            return what_should_i_yell(
                lhs_monkey.clone(),
                monkey_to_nr,
                monkey_to_function,
                Some(new_nr_to_produce),
            );
        } else {
            panic!("one side should be none");
        }
    }
}

fn find_out_monkey_nr(
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
    result
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
    Ok(find_out_monkey_nr(
        "root".to_string(),
        &mut monkey_to_nr,
        &monkey_to_function,
    ))
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

    let nr_of_rhs_monkey: Option<i64> = what_should_i_yell(
        monkey_to_function["root"].rhs.clone(),
        &mut monkey_to_nr,
        &monkey_to_function,
        None,
    );
    println!("\n{}", nr_of_rhs_monkey.unwrap());
    Ok(what_should_i_yell(
        monkey_to_function["root"].lhs.clone(),
        &mut monkey_to_nr,
        &monkey_to_function,
        Some(nr_of_rhs_monkey.unwrap()),
    )
    .unwrap())
}
/*
   let nr_of_rhs_monkey: Option<i64> = what_should_i_yell(
   "root".to_string(),
   &mut monkey_to_nr,
   &monkey_to_function,
   None,
   );
   println!("{:?}", nr_of_rhs_monkey);
   Ok(what_should_i_yell(
   "root".to_string(),
   &mut monkey_to_nr,
   &monkey_to_function,
   Some(nr_of_rhs_monkey.unwrap()),
   )
   .unwrap())
   }

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
