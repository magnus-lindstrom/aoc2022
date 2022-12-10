use crate::utils;

const FILE_PATH: &str = "inputs/day10.txt";

pub fn result_a() -> Result<i32, &'static str> {
    let input: Vec<Vec<String>> = utils::vector_of_string_vectors_from_file(FILE_PATH);
    let interesting_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut reg_x: i32 = 1;
    let mut output: i32 = 0;
    let mut cycle_nr: i32 = 0;
    let mut cycles_to_add: i32;
    let mut reg_value_to_add: i32;

    for operation in input.iter() {
        if operation[0] == "noop" {
            cycles_to_add = 1;
            reg_value_to_add = 0;
        } else if operation[0] == "addx" {
            cycles_to_add = 2;
            reg_value_to_add = operation[1]
                .parse::<i32>()
                .expect(format!("Could not convert {} to i32", operation[1]).as_str());
        } else {
            panic!("operation not implemented");
        }

        for _ in 0..cycles_to_add {
            cycle_nr += 1;
            if interesting_cycles.contains(&(cycle_nr as i32)) {
                output += cycle_nr as i32 * reg_x;
            }
        }
        reg_x += reg_value_to_add;
    }
    Ok(output)
}

pub fn result_b() -> Result<String, &'static str> {
    let input: Vec<Vec<String>> = utils::vector_of_string_vectors_from_file(FILE_PATH);
    let mut reg_x: i32 = 1;
    let mut output: String = "".to_string();
    let mut pixel_position: i32 = -1; // So that on first cycle, the pix pos is 0
    let mut cycles_to_add: i32;
    let mut reg_value_to_add: i32;

    for operation in input.iter() {
        if operation[0] == "noop" {
            cycles_to_add = 1;
            reg_value_to_add = 0;
        } else if operation[0] == "addx" {
            cycles_to_add = 2;
            reg_value_to_add = operation[1]
                .parse::<i32>()
                .expect(format!("Could not convert {} to i32", operation[1]).as_str());
        } else {
            panic!("operation not implemented");
        }

        for _ in 0..cycles_to_add {
            pixel_position += 1;
            if pixel_position == 40 {
                output.push_str("\n");
                pixel_position = 0;
            }
            if pixel_position <= (reg_x + 1) && pixel_position >= (reg_x - 1) {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        }
        reg_x += reg_value_to_add;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 15360);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        let exp_answer = "###..#..#.#....#..#...##..##..####..##..\n\
                          #..#.#..#.#....#..#....#.#..#....#.#..#.\n\
                          #..#.####.#....####....#.#......#..#..#.\n\
                          ###..#..#.#....#..#....#.#.##..#...####.\n\
                          #....#..#.#....#..#.#..#.#..#.#....#..#.\n\
                          #....#..#.####.#..#..##...###.####.#..#.";
        assert_eq!(answer, exp_answer);
    }
}
