use crate::utils;
use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day25.txt";

fn snafu_to_dec(string: String) -> i64 {
    let mut dec: i64 = 0;
    let map: HashMap<char, i64> =
        HashMap::from([('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2)]);
    for (i_ch, ch) in string.chars().rev().enumerate() {
        dec += map[&ch] * 5_i64.pow(i_ch as u32);
    }
    dec
}

fn can_create_num_with_lesser_i(x: i64, i: i64) -> bool {
    let mut sum: i64 = 0;
    for j in 0..i {
        sum += 2 * 5_i64.pow(j as u32);
    }
    if x <= 0 && x >= -sum {
        true
    } else if x >= 0 && x <= sum {
        true
    } else {
        false
    }
}

fn decimal_to_snafu(x: i64) -> String {
    let dec_to_snafu: HashMap<i64, char> =
        HashMap::from([(2, '2'), (1, '1'), (0, '0'), (-1, '-'), (-2, '=')]);
    let mut result: String = "".to_string();
    let mut started: bool = false;
    let mut nr: i64 = x;
    'outer: for i in (0..21).rev() {
        if nr == 0 {
            result.push('0');
        } else if nr > 0 {
            for j in (1..=2).rev() {
                let to_remove = j * 5_i64.pow(i as u32);
                if can_create_num_with_lesser_i(nr - to_remove, i) {
                    nr -= to_remove;
                    result.push(dec_to_snafu[&(j as i64)]);
                    started = true;
                    continue 'outer;
                }
            }
            if started {
                result.push('0'); // just add 0 if 1 and 2 got too much
            }
        } else {
            for j in (1..=2).rev() {
                let to_add = j * 5_i64.pow(i as u32);
                if can_create_num_with_lesser_i(nr + to_add, i) {
                    nr += to_add;
                    result.push(dec_to_snafu[&(-j as i64)]);
                    continue 'outer;
                }
            }
            result.push('0'); // just add 0 if -1 and -2 got too much
        }
    }
    result
}

pub fn result_a() -> Result<String, &'static str> {
    let input = utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
    let mut sum: i64 = 0;
    for line in input.iter() {
        sum += snafu_to_dec(line.to_string());
    }

    Ok(decimal_to_snafu(sum))
}

pub fn result_b() -> Result<String, &'static str> {
    Ok("freebie".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, "2-21=02=1-121-2-11-0");
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, "freebie");
    }
}
