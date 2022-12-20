use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day20.txt";
//const FILE_PATH: &str = "inputs/day20_test.txt";

fn get_new_pos_of_nr(a: i32, mut b: i32, input_len: i32) -> i32 {
    let new_place: i32;
    if a + b <= -input_len {
        b += input_len;
    }

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
    a: i32,
    b: i32,
    input_len: i32,
) -> (Vec<i32>, bool) {
    let intermediate_positions: Vec<i32>;
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

fn get_new_order(old_vec: &Vec<i32>, old_pos_to_new_pos: &HashMap<i32, i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![0; old_vec.len()];
    for i in 0..old_vec.len() {
        let nr = old_vec[i];
        let new_pos = old_pos_to_new_pos[&(i as i32)];
        new_vec[new_pos as usize] = nr;
    }

    new_vec
}

fn get_num_x_positions_after_0(vec: &Vec<i32>, x: usize) -> i32 {
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

pub fn result_a() -> Result<i32, &'static str> {
    /*
    println!(
    "affected_positions: {:?}",
    get_affected_positions(2, -5, 20)
    );
    return Ok(0);
    */
    let input: Vec<i32> = crate::utils::nr_vec_from_file_allow_empty_lines(FILE_PATH);
    let input_len = input.len() as i32;
    let mut old_pos_to_new_pos: HashMap<i32, i32> = HashMap::new();
    let mut new_pos_to_old_pos: HashMap<i32, i32> = HashMap::new();
    for (i, _) in input.iter().enumerate() {
        old_pos_to_new_pos.insert(i as i32, i as i32);
        new_pos_to_old_pos.insert(i as i32, i as i32);
    }
    for (prior_pos, item) in input.iter().enumerate() {
        let current_pos_in_list = old_pos_to_new_pos[&(prior_pos as i32)];
        let updated_position = get_new_pos_of_nr(current_pos_in_list, *item, input_len);
        let (intermediate_positions, moving_to_the_right) =
            get_affected_positions_and_if_moving_to_the_right(
                old_pos_to_new_pos[&(prior_pos as i32)],
                *item,
                input_len,
            );
        let update_intermediates_with: i32;
        if moving_to_the_right {
            update_intermediates_with = -1;
        } else {
            update_intermediates_with = 1;
        }

        //println!("intermediate_positions: {:?}", intermediate_positions);
        for inter_pos in intermediate_positions.iter() {
            // moving all numbers affected by the actual number move
            let new_inter_pos = (inter_pos + input_len + update_intermediates_with) % input_len;
            let input_pos = new_pos_to_old_pos[&inter_pos];
            new_pos_to_old_pos.insert(new_inter_pos, input_pos);

            // update old to new position map
            old_pos_to_new_pos.insert(input_pos, new_inter_pos);
        }

        // now update the nr in question
        old_pos_to_new_pos.insert(prior_pos as i32, updated_position);
        new_pos_to_old_pos.insert(updated_position, prior_pos as i32);
    }
    let new_order = get_new_order(&input, &old_pos_to_new_pos);
    println!("{:?}", new_order);

    let mut result_sum: i32 = 0;
    result_sum += get_num_x_positions_after_0(&new_order, 1000);
    result_sum += get_num_x_positions_after_0(&new_order, 2000);
    result_sum += get_num_x_positions_after_0(&new_order, 3000);

    Ok(result_sum)
}

pub fn result_b() -> Result<i32, &'static str> {
    Ok(0)
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
