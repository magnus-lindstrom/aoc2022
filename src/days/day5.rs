use std::fs;

const FILE_PATH: &str = "inputs/day5.txt";

fn get_original_stacks() -> Vec<Vec<char>> {
    let file_contents: String = fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());

    let last_stack_line = 8;
    let nr_stacks = 9;
    let chars_between_stack_objects = 4; // objects being the chars in the stacks

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
                _ => stacks[i_stack].push(ch),
            }
        }
    }
    return stacks;
}

#[allow(dead_code)]
fn print_stacks(stacks: &Vec<Vec<char>>) -> () {
    for s in stacks {
        println!("{:?}", s);
    }
}

fn top_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut return_string: String = "".to_string();
    for s in stacks {
        return_string.push(s[0]);
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
            let item = stacks[from_stack - 1][0];
            stacks[to_stack - 1].insert(0, item);

            stacks[from_stack - 1].remove(0);
        }
    }

    Ok(top_of_stacks(&stacks))
}

pub fn result_b() -> Result<i32, &'static str> {
    let _file_path = "inputs/dayX.txt";
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, "TDCHVHJTG");
    }

    /*
    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 0);
    }
    */
}
