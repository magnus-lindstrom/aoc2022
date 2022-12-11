use crate::utils;
use std::collections::VecDeque;

const FILE_PATH: &str = "inputs/day11.txt";
const TEST_FILE_PATH: &str = "inputs/day11_test.txt";

#[derive(Debug)]
struct Item {
    worry_lvl: i32,
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    Uninitialized,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    operation_number: String,
    divisor: i32,
    true_target: usize,
    false_target: usize,
    inspections_carried_out: i32,
    reduction_nr: i64,
}
impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: Operation::Uninitialized,
            operation_number: "".to_string(),
            divisor: 0,
            true_target: 0,
            false_target: 0,
            inspections_carried_out: 0,
            reduction_nr: 0,
        }
    }
    fn add_item(&mut self, item: &str) -> () {
        let worry_lvl: i32 = item
            .parse()
            .expect(format!("Could not parse {} as i32.", item).as_str());
        self.items.push_back(Item { worry_lvl });
    }
    fn inspect_item(&mut self, divide: bool) -> () {
        let actual_operation_nr: i32 = match self.operation_number.as_str() {
            "old" => self.items[0].worry_lvl,
            _ => self.operation_number.parse().unwrap(),
        };
        let would_be_result: i64;
        match self.operation {
            Operation::Addition => {
                would_be_result = self.items[0].worry_lvl as i64 + actual_operation_nr as i64;
            }
            Operation::Multiplication => {
                would_be_result = self.items[0].worry_lvl as i64 * actual_operation_nr as i64;
            }
            Operation::Uninitialized => panic!("Uninitialized operation"),
        }
        self.items[0].worry_lvl = would_be_result as i32;
        if divide {
            self.items[0].worry_lvl /= 3;
        } else {
            self.items[0].worry_lvl = reduce_item_worry_lvl(would_be_result, self.reduction_nr);
        }
        self.inspections_carried_out += 1;
    }
    fn throw_item_at_target(&mut self) -> (Item, usize) {
        let target_monkey: usize;
        if self.items[0].worry_lvl % self.divisor == 0 {
            target_monkey = self.true_target;
        } else {
            target_monkey = self.false_target;
        }

        let item: Item = self.items.pop_front().expect("Nothing to pop");
        (item, target_monkey)
    }
}

fn reduce_item_worry_lvl(mut worry_lvl: i64, reduction_nr: i64) -> i32 {
    while worry_lvl > reduction_nr {
        worry_lvl -= reduction_nr;
    }
    worry_lvl as i32
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
            monkeys.push(Monkey::new());
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
            let divisor: i32 = line[3].parse().unwrap();
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

    let mut reduction_nr: i64 = 1;
    let mut reduction_nr_set: Vec<i64> = Vec::new();
    for monkey in monkeys.iter() {
        if !reduction_nr_set.contains(&(monkey.divisor as i64)) {
            reduction_nr_set.push(monkey.divisor as i64);
        }
    }
    for nr in reduction_nr_set.iter() {
        reduction_nr *= nr;
    }

    for monkey in monkeys.iter_mut() {
        monkey.reduction_nr = reduction_nr;
    }
    monkeys
}

pub fn result_a() -> Result<i64, &'static str> {
    let input: Vec<Vec<String>> = utils::vector_of_string_vectors_from_file(FILE_PATH);
    let nr_rounds: i32 = 20;
    let mut monkeys: Vec<Monkey> = get_monkeys(input);

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

/// Running this function with the true input takes > 5min
/// Test input takes < 1s
pub fn result_b(use_test_input: bool) -> Result<i64, &'static str> {
    let input: Vec<Vec<String>>;
    if use_test_input {
        input = utils::vector_of_string_vectors_from_file(TEST_FILE_PATH);
    } else {
        input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    }
    let nr_rounds: i32 = 10000;
    let mut monkeys: Vec<Monkey> = get_monkeys(input);

    for _ in 0..nr_rounds {
        for i_monkey in 0..monkeys.len() {
            // for i_monkey in 0..3 {
            while monkeys[i_monkey].items.len() > 0 {
                monkeys[i_monkey].inspect_item(false);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 66124);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b(true).unwrap();
        assert_eq!(answer, 2713310158);
    }
}
