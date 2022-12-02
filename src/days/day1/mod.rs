use crate::utils;


pub fn question_a_answer() -> i32 {

    let file_path = "inputs/day1.txt";
    let numbers = utils::file_name_to_i32_vec_allow_empty_lines(file_path);

    let mut max_cal = 0;

    let mut tmp_cal_sum = 0;
    for cal in numbers.iter() {
        if *cal == 0 {
            if tmp_cal_sum > max_cal {
                max_cal = tmp_cal_sum;
            }
            tmp_cal_sum = 0;
        }
        else if *cal > 0 { tmp_cal_sum += cal; }
        else { panic!("Calorie count smaller than 0: {}", cal); }
    }
    max_cal
}

pub fn question_b_answer() -> i32 {

    let file_path = "inputs/day1.txt";
    let numbers = utils::file_name_to_i32_vec_allow_empty_lines(file_path);

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
        }
        else if *cal > 0 { tmp_cal_sum += cal; }
        else { panic!("Calorie count smaller than 0: {}", cal); }
    }
    highest_cal_sum + second_highest_cal_sum + third_highest_cal_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_a_is_correct() {
        let answer = question_a_answer();
        assert_eq!(answer, 69310);
    }

    #[test]
    fn question_b_is_correct() {
        let answer = question_b_answer();
        assert_eq!(answer, 206104);
    }
}
