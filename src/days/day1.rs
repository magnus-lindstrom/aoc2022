use crate::utils;

pub fn result_a() -> Result<i32, &'static str> {
    /*
     * Input has rows of numbers, where some rows are empty.
     * Lines with consecutive numbers in them represent a sum.
     * Find out what the biggest sum is.
     */

    let file_path = "inputs/day1.txt";
    let numbers: Vec<i32> = utils::nr_vec_from_file_allow_empty_lines(file_path);

    let mut max_cal = 0;

    let mut tmp_cal_sum = 0;
    for cal in numbers.iter() {
        if *cal == 0 {
            if tmp_cal_sum > max_cal {
                max_cal = tmp_cal_sum;
            }
            tmp_cal_sum = 0;
        } else if *cal > 0 {
            tmp_cal_sum += cal;
        } else {
            panic!("Calorie count smaller than 0: {}", cal);
        }
    }
    Ok(max_cal)
}

pub fn result_b() -> Result<i32, &'static str> {
    /*
     * Input has rows of numbers, where some rows are empty.
     * Lines with consecutive numbers in them represent a sum.
     * Find out what the sum of the three biggest sums are.
     */

    let file_path = "inputs/day1.txt";
    let numbers: Vec<i32> = utils::nr_vec_from_file_allow_empty_lines(file_path);

    let mut highest_cal_sum = 0;
    let mut second_highest_cal_sum = 0;
    let mut third_highest_cal_sum = 0;

    let mut tmp_cal_sum = 0;
    for cal in numbers.iter() {
        if *cal == 0 {
            if tmp_cal_sum > highest_cal_sum {
                third_highest_cal_sum = second_highest_cal_sum;
                second_highest_cal_sum = highest_cal_sum;
                highest_cal_sum = tmp_cal_sum;
            } else if tmp_cal_sum > second_highest_cal_sum {
                third_highest_cal_sum = second_highest_cal_sum;
                second_highest_cal_sum = tmp_cal_sum;
            } else if tmp_cal_sum > third_highest_cal_sum {
                third_highest_cal_sum = tmp_cal_sum;
            }
            tmp_cal_sum = 0;
        } else if *cal > 0 {
            tmp_cal_sum += cal;
        } else {
            panic!("Calorie count smaller than 0: {}", cal);
        }
    }
    Ok(highest_cal_sum + second_highest_cal_sum + third_highest_cal_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a();
        assert_eq!(answer, Ok(69310));
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b();
        assert_eq!(answer, Ok(206104));
    }
}
