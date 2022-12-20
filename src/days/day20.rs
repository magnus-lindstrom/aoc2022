use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day20.txt";

fn get_new_pos_of_nr(a: i64, mut b: i64, input_len: i64) -> i64 {
    let new_place: i64;

    b = b % (input_len - 1);

    if a + b <= 0 {
        new_place = (a + b + input_len - 1) % input_len;
    } else if a + b >= input_len {
        new_place = (a + b + input_len + 1) % input_len;
    } else {
        new_place = (a + b + input_len) % input_len;
    }
    new_place
}

fn get_affected_positions_and_if_moving_to_the_right(
    a: i64,
    b: i64,
    input_len: i64,
) -> (Vec<i64>, bool) {
    let intermediate_positions: Vec<i64>;
    let moving_to_the_right: bool;

    let new_pos = get_new_pos_of_nr(a, b, input_len);
    if new_pos < a {
        intermediate_positions = (new_pos..a).map(|e| e % input_len).rev().collect();
        moving_to_the_right = false;
    } else {
        intermediate_positions = (a + 1..=new_pos).map(|e| e % input_len).collect();
        moving_to_the_right = true;
    }

    return (intermediate_positions, moving_to_the_right);
}

fn get_new_order(old_vec: &Vec<i64>, old_pos_to_new_pos: &HashMap<i64, i64>) -> Vec<i64> {
    let mut new_vec: Vec<i64> = vec![0; old_vec.len()];
    for i in 0..old_vec.len() {
        let nr = old_vec[i];
        let new_pos = old_pos_to_new_pos[&(i as i64)];
        new_vec[new_pos as usize] = nr;
    }

    new_vec
}

fn get_num_x_positions_after_0(vec: &Vec<i64>, x: usize) -> i64 {
    let vec_len = vec.len();
    let mut pos_0: Option<usize> = None;
    for i in 0..vec_len {
        if vec[i] == 0 {
            pos_0 = Some(i);
        }
    }
    match pos_0 {
        Some(pos) => return vec[(pos + x) % vec_len],
        None => panic!("Did not find the zero"),
    }
}

pub fn result_a() -> Result<i64, &'static str> {
    let input: Vec<i64> = crate::utils::nr_vec_from_file_allow_empty_lines(FILE_PATH);
    let input_len = input.len();
    let mut old_pos_to_new_pos: HashMap<i64, i64> = HashMap::new();
    let mut new_pos_to_old_pos: HashMap<i64, i64> = HashMap::new();
    for (i, _) in input.iter().enumerate() {
        old_pos_to_new_pos.insert(i as i64, i as i64);
        new_pos_to_old_pos.insert(i as i64, i as i64);
    }
    for (prior_pos, item) in input.iter().enumerate() {
        let current_pos_in_list = old_pos_to_new_pos[&(prior_pos as i64)];
        let updated_position = get_new_pos_of_nr(current_pos_in_list, *item, input_len as i64);
        let (intermediate_positions, moving_to_the_right) =
            get_affected_positions_and_if_moving_to_the_right(
                old_pos_to_new_pos[&(prior_pos as i64)],
                *item,
                input_len as i64,
            );
        let update_intermediates_with: i64;
        if moving_to_the_right {
            update_intermediates_with = -1;
        } else {
            update_intermediates_with = 1;
        }

        for inter_pos in intermediate_positions.iter() {
            // moving all numbers affected by the actual number move
            let new_inter_pos =
                (inter_pos + input_len as i64 + update_intermediates_with) % input_len as i64;
            let input_pos = new_pos_to_old_pos[&inter_pos];
            new_pos_to_old_pos.insert(new_inter_pos, input_pos);

            // update old to new position map
            old_pos_to_new_pos.insert(input_pos, new_inter_pos);
        }

        // now update the nr in question
        old_pos_to_new_pos.insert(prior_pos as i64, updated_position);
        new_pos_to_old_pos.insert(updated_position, prior_pos as i64);
        //println!("{:?}", get_new_order(&input, &old_pos_to_new_pos));
    }
    let new_order = get_new_order(&input, &old_pos_to_new_pos);

    let mut result_sum: i64 = 0;
    result_sum += get_num_x_positions_after_0(&new_order, 1000);
    result_sum += get_num_x_positions_after_0(&new_order, 2000);
    result_sum += get_num_x_positions_after_0(&new_order, 3000);

    Ok(result_sum)
}

pub fn result_b() -> Result<i64, &'static str> {
    let mut input: Vec<i64> = crate::utils::nr_vec_from_file_allow_empty_lines(FILE_PATH);
    let magic_nr: i64 = 811589153;
    let input_len = input.len();
    for i in 0..input_len {
        input[i] *= magic_nr;
    }
    let mut old_pos_to_new_pos: HashMap<i64, i64> = HashMap::new();
    let mut new_pos_to_old_pos: HashMap<i64, i64> = HashMap::new();
    for (i, _) in input.iter().enumerate() {
        old_pos_to_new_pos.insert(i as i64, i as i64);
        new_pos_to_old_pos.insert(i as i64, i as i64);
    }
    let mut new_order = input.clone();
    for _ in 0..10 {
        for (prior_pos, item) in input.iter().enumerate() {
            let current_pos_in_list = old_pos_to_new_pos[&(prior_pos as i64)];
            let updated_position = get_new_pos_of_nr(current_pos_in_list, *item, input_len as i64);
            let (intermediate_positions, moving_to_the_right) =
                get_affected_positions_and_if_moving_to_the_right(
                    old_pos_to_new_pos[&(prior_pos as i64)],
                    *item,
                    input_len as i64,
                );
            let update_intermediates_with: i64;
            if moving_to_the_right {
                update_intermediates_with = -1;
            } else {
                update_intermediates_with = 1;
            }

            for inter_pos in intermediate_positions.iter() {
                // moving all numbers affected by the actual number move
                let new_inter_pos =
                    (inter_pos + input_len as i64 + update_intermediates_with) % input_len as i64;
                let input_pos = new_pos_to_old_pos[&inter_pos];
                new_pos_to_old_pos.insert(new_inter_pos, input_pos);

                // update old to new position map
                old_pos_to_new_pos.insert(input_pos, new_inter_pos);
            }

            // now update the nr in question
            old_pos_to_new_pos.insert(prior_pos as i64, updated_position);
            new_pos_to_old_pos.insert(updated_position, prior_pos as i64);
        }
        new_order = get_new_order(&input, &old_pos_to_new_pos);
    }

    let mut result_sum: i64 = 0;
    result_sum += get_num_x_positions_after_0(&new_order, 1000);
    result_sum += get_num_x_positions_after_0(&new_order, 2000);
    result_sum += get_num_x_positions_after_0(&new_order, 3000);

    Ok(result_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 9866);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 12374299815791);
    }
}
