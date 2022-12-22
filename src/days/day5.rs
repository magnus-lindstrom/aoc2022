use std::fs;

const FILE_PATH: &str = "inputs/day5.txt";

fn get_original_stacks() -> Vec<Vec<char>> {
    /*
     * The input file begins like this:
     *
     * [F]         [L]     [M]
     * [T]     [H] [V] [G] [V]
     * [N]     [T] [D] [R] [N]     [D]
     * [Z]     [B] [C] [P] [B] [R] [Z]
     * [M]     [J] [N] [M] [F] [M] [V] [H]
     * [G] [J] [L] [J] [S] [C] [G] [M] [F]
     * [H] [W] [V] [P] [W] [H] [H] [N] [N]
     * [J] [V] [G] [B] [F] [G] [D] [H] [G]
     *  1   2   3   4   5   6   7   8   9
     *
     * move 6 from 4 to 3
     * move 5 from 8 to 9
     * move 1 from 4 to 5
     * ....
     */
    let file_contents: String = fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());

    let last_stack_line = 8;
    let nr_stacks = 9;
    let chars_between_stack_objects = 4;

    // The first element in each vector represents the bottom element in the corresponding stack
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..nr_stacks {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }

    for (i_line, line) in file_contents.lines().enumerate() {
        if i_line == last_stack_line {
            break;
        }
        for i_stack in 0..nr_stacks {
            let ch = line
                .chars()
                .nth(i_stack * chars_between_stack_objects + 1)
                .expect("Could not find nth char in line");
            match ch {
                ' ' => (),
                _ => stacks[i_stack].insert(0, ch),
            }
        }
    }
    return stacks;
}

fn top_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut return_string: String = "".to_string();
    for s in stacks {
        return_string.push(*s.last().expect("can not get last element of empty string"));
    }
    return_string
}

pub fn result_a() -> Result<String, &'static str> {
    let mut stacks = get_original_stacks();

    let file_contents: String = fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());
    let starting_line_of_move_segment = 11;
    for line in file_contents
        .lines()
        .enumerate()
        .filter(|(i, _)| i >= &(starting_line_of_move_segment - 1))
        .map(|(_, l)| l)
    {
        let words: Vec<&str> = line.split_whitespace().collect();
        let quantity: i32 = words[1].parse().unwrap();
        let from_stack: usize = words[3].parse().unwrap();
        let to_stack: usize = words[5].parse().unwrap();
        for _ in 0..quantity {
            let item = stacks[from_stack - 1]
                .pop()
                .expect("stack is empty, can not pop");
            stacks[to_stack - 1].push(item);
        }
    }

    Ok(top_of_stacks(&stacks))
}

pub fn result_b() -> Result<String, &'static str> {
    let _file_path = "inputs/dayX.txt";
    let mut stacks = get_original_stacks();

    let file_contents: String = fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());
    let starting_line_of_move_segment = 11;
    for line in file_contents
        .lines()
        .enumerate()
        .filter(|(i, _)| i >= &(starting_line_of_move_segment - 1))
        .map(|(_, l)| l)
    {
        let words: Vec<&str> = line.split_whitespace().collect();
        let quantity: i32 = words[1].parse().unwrap();
        let from_stack: usize = words[3].parse().unwrap();
        let to_stack: usize = words[5].parse().unwrap();

        // Move items one by one to a temporary stack, and then from there to the real destination,
        // so the order is preserved.
        let mut tmp_stack: Vec<char> = Vec::new();
        for _ in 0..quantity {
            let item = stacks[from_stack - 1]
                .pop()
                .expect("stack is empty, can not pop");
            tmp_stack.push(item);
        }
        for _ in 0..quantity {
            let item = tmp_stack.pop().expect("stack is empty, can not pop");
            stacks[to_stack - 1].push(item);
        }
    }

    Ok(top_of_stacks(&stacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, "TDCHVHJTG");
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, "NGCMPJLHV");
    }
}
