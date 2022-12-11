use crate::utils;
use num_format::{Locale, ToFormattedString};
use std::collections::VecDeque;

const FILE_PATH: &str = "inputs/day11.txt";
const TEST_FILE_PATH: &str = "inputs/day11_test.txt";

#[derive(Debug)]
struct Item {
    worry_lvl: i64,
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    Uninitialized,
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: VecDeque<Item>,
    operation: Operation,
    operation_number: String,
    divisor: i64,
    true_target: usize,
    false_target: usize,
    inspections_carried_out: i32,
}
impl Monkey {
    fn new(id: usize) -> Monkey {
        Monkey {
            id: id as i32,
            items: VecDeque::new(),
            operation: Operation::Uninitialized,
            operation_number: "".to_string(),
            divisor: 0,
            true_target: 0,
            false_target: 0,
            inspections_carried_out: 0,
        }
    }
    fn add_item(&mut self, item: &str) -> () {
        let worry_lvl: i64 = item
            .parse()
            .expect(format!("Could not parse {} as i32.", item).as_str());
        self.items.push_back(Item { worry_lvl });
    }
    fn inspect_item(&mut self, divide: bool) -> () {
        // println!("monkey {} inspecting. Before: {:?}", self.id, self.items);
        let actual_operation_nr: i64 = match self.operation_number.as_str() {
            "old" => self.items[0].worry_lvl,
            _ => self.operation_number.parse().unwrap(),
        };
        match self.operation {
            Operation::Addition => {
                self.items[0].worry_lvl += actual_operation_nr;
            }
            Operation::Multiplication => {
                self.items[0].worry_lvl *= actual_operation_nr;
            }
            Operation::Uninitialized => panic!("Uninitialized operation"),
        }
        // if would_be_result > std::i32::MAX as i64 {
        // let panic_str: String = format!(
        // "{} * {} would be {}, when max is {}",
        // self.items[0].worry_lvl.to_formatted_string(&Locale::en),
        // actual_operation_nr.to_formatted_string(&Locale::en),
        // would_be_result.to_formatted_string(&Locale::en),
        // std::i32::MAX.to_formatted_string(&Locale::en)
        // );
        // panic!("{}", panic_str);
        // }
        if divide {
            self.items[0].worry_lvl /= 3;
        }
        self.inspections_carried_out += 1;
        // println!("monkey {} inspecting. After: {:?}", self.id, self.items);
    }
    fn throw_item_at_target(&mut self) -> (Item, usize) {
        let target_monkey: usize;
        // println!(
        // "monkey {}. {} divisible by {}: {}",
        // self.id,
        // self.items[0].worry_lvl,
        // self.divisor,
        // self.items[0].worry_lvl % self.divisor == 0
        // );
        if self.items[0].worry_lvl % self.divisor == 0 {
            target_monkey = self.true_target;
        } else {
            target_monkey = self.false_target;
        }

        let item: Item = self.items.pop_front().expect("Nothing to pop");
        // println!(
        // "monkey {} throwing item with worry lvl {} to {}\n",
        // self.id, item.worry_lvl, target_monkey
        // );
        (item, target_monkey)
    }

    fn reduce_item_worry_levels(&mut self, reduction: i64) -> () {
        // println!("before red {:?}", self.items);
        for i in 0..self.items.len() {
            while self.items[i].worry_lvl > reduction {
                self.items[i].worry_lvl -= reduction;
            }
        }
        // println!("after red {:?}", self.items);
    }
}

fn _is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need
                          // to use `return`
        }
    }
    true // last value to return
}

fn get_monkey_business(monkeys: Vec<Monkey>) -> i64 {
    let mut highest_inspection_rate: i64 = 0;
    let mut second_highest_inspection_rate: i64 = 0;
    for i in 0..monkeys.len() {
        if monkeys[i].inspections_carried_out as i64 > highest_inspection_rate {
            second_highest_inspection_rate = highest_inspection_rate;
            highest_inspection_rate = monkeys[i].inspections_carried_out as i64;
        } else if monkeys[i].inspections_carried_out as i64 > second_highest_inspection_rate {
            second_highest_inspection_rate = monkeys[i].inspections_carried_out as i64;
        }
    }
    highest_inspection_rate * second_highest_inspection_rate
}

fn get_monkeys(input: Vec<Vec<String>>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in input.iter() {
        if line.len() == 0 {
        } else if line[0] == "Monkey".to_string() {
            monkeys.push(Monkey::new(monkeys.len()));
        } else if line[0] == "Starting".to_string() && line[1] == "items:".to_string() {
            for item in line[2..].iter() {
                monkeys
                    .last_mut()
                    .expect("Could not find last monkey in vector")
                    .add_item(item.trim_end_matches(","));
            }
        } else if line[0] == "Operation:".to_string() {
            let operation: Operation;
            if line[4] == "*".to_string() {
                operation = Operation::Multiplication;
            } else if line[4] == "+".to_string() {
                operation = Operation::Addition;
            } else {
                panic!("Strange operation");
            }

            let operation_number: String = line[5].clone();
            monkeys
                .last_mut()
                .expect("Could not find last monkey in vector")
                .operation = operation;
            monkeys
                .last_mut()
                .expect("Could not find last monkey in vector")
                .operation_number = operation_number;
        } else if line[0] == "Test:".to_string() {
            let divisor: i64 = line[3].parse().unwrap();
            monkeys
                .last_mut()
                .expect("Could not find last monkey in vector")
                .divisor = divisor;
        } else if line[0] == "If".to_string() && line[1] == "true:".to_string() {
            let true_target: usize = line[5].parse().unwrap();
            monkeys
                .last_mut()
                .expect("Could not find last monkey in vector")
                .true_target = true_target;
        } else if line[0] == "If".to_string() && line[1] == "false:".to_string() {
            let false_target: usize = line[5].parse().unwrap();
            monkeys
                .last_mut()
                .expect("Could not find last monkey in vector")
                .false_target = false_target;
        } else {
            println!("{:?}", line);
            panic!("Unexpected line");
        }
    }
    monkeys
}

pub fn result_a() -> Result<i64, &'static str> {
    let input: Vec<Vec<String>> = utils::vector_of_string_vectors_from_file(FILE_PATH);
    let nr_rounds: i32 = 20;
    let mut monkeys: Vec<Monkey> = get_monkeys(input);

    // for i in 0..monkeys.len() {
    // println!("{:?}", monkeys[i]);
    // }
    for _ in 0..nr_rounds {
        for i_monkey in 0..monkeys.len() {
            // for i_monkey in 0..3 {
            while monkeys[i_monkey].items.len() > 0 {
                monkeys[i_monkey].inspect_item(true);
                let thrown_item: Item;
                let target_monkey: usize;
                (thrown_item, target_monkey) = monkeys[i_monkey].throw_item_at_target();
                monkeys[target_monkey].items.push_back(thrown_item);
            }
        }
    }

    let monkey_business: i64 = get_monkey_business(monkeys);
    Ok(monkey_business)
}

pub fn result_b() -> Result<i64, &'static str> {
    let input: Vec<Vec<String>> = utils::vector_of_string_vectors_from_file(FILE_PATH);
    let nr_rounds: i32 = 10000;
    let mut monkeys: Vec<Monkey> = get_monkeys(input);
    let mut reduction: i64 = 1;
    let mut reduction_set: Vec<i64> = Vec::new();
    for monkey in monkeys.iter() {
        if !reduction_set.contains(&monkey.divisor) {
            reduction_set.push(monkey.divisor);
        }
    }
    for nr in reduction_set.iter() {
        reduction *= nr;
    }

    // for i in 0..monkeys.len() {
    // println!("{:?}", monkeys[i]);
    // }
    for _ in 0..nr_rounds {
        for i_monkey in 0..monkeys.len() {
            // for i_monkey in 0..3 {
            monkeys[i_monkey].reduce_item_worry_levels(reduction);
            while monkeys[i_monkey].items.len() > 0 {
                monkeys[i_monkey].inspect_item(false);
                let thrown_item: Item;
                let target_monkey: usize;
                (thrown_item, target_monkey) = monkeys[i_monkey].throw_item_at_target();
                monkeys[target_monkey].items.push_back(thrown_item);
            }
        }
    }

    println!("");
    for i in 0..monkeys.len() {
        println!(
            "{} has {} inspections",
            monkeys[i].id, monkeys[i].inspections_carried_out
        );
    }
    let monkey_business: i64 = get_monkey_business(monkeys);
    Ok(monkey_business)
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
