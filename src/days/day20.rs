use std::collections::HashMap;
//const FILE_PATH: &str = "inputs/day20.txt";
const FILE_PATH: &str = "inputs/day20_test.txt";

fn get_affected_positions(a: i32, b: i32, input_len: i32) -> Vec<i32> {
    let intermediate_positions: Vec<i32>;
    // +1 to not take the position itself
    if b > 0 {
        intermediate_positions = (a + 1..=a + b).map(|e| e % input_len).collect();
    } else if b < 0 {
        assert_eq!(b.abs() < input_len, true);
        intermediate_positions = (input_len + a + b..input_len + a)
            .map(|e| e % input_len)
            .collect();
    } else {
        intermediate_positions = Vec::new();
    }

    return intermediate_positions;
}

fn print_new_order(old_vec: &Vec<i32>, old_pos_to_new_pos: &HashMap<i32, i32>) -> () {
    let mut new_vec: Vec<i32> = vec![0; old_vec.len()];
    for i in 0..old_vec.len() {
        let nr = old_vec[i];
        let new_pos = old_pos_to_new_pos[&(i as i32)];
        new_vec[new_pos as usize] = nr;
    }

    println!("{:?}", new_vec);
}

pub fn result_a() -> Result<i32, &'static str> {
    /*
    println!(
        "affected_positions: {:?}",
        get_affected_positions(2, -5, 20)
    );
    */
    let input: Vec<i32> = crate::utils::nr_vec_from_file_allow_empty_lines(FILE_PATH);
    println!("original input: {:?}", input);
    let input_len = input.len() as i32;
    let mut old_pos_to_new_pos: HashMap<i32, i32> = HashMap::new();
    let mut new_pos_to_old_pos: HashMap<i32, i32> = HashMap::new();
    for (i, _) in input.iter().enumerate() {
        old_pos_to_new_pos.insert(i as i32, i as i32);
        new_pos_to_old_pos.insert(i as i32, i as i32);
    }
    for (prior_pos, item) in input.iter().enumerate() {
        println!("moving number {}", item);
        let updated_position =
            (old_pos_to_new_pos[&(prior_pos as i32)] + item + input_len) % input_len;
        let intermediate_positions: Vec<i32> =
            get_affected_positions(old_pos_to_new_pos[&(prior_pos as i32)], *item, input_len);
        let update_intermediates_with: i32;
        if item < &0 {
            update_intermediates_with = 1;
        } else {
            update_intermediates_with = -1;
        }
        println!("intermediate_positions: {:?}", intermediate_positions);
        for intermediate_position in intermediate_positions.iter() {
            let new_intermediate_position =
                (intermediate_position + input_len + update_intermediates_with) % input_len;
            //println!(
            //"pos {} becomes pos {}",
            //intermediate_position, new_intermediate_position
            //);
            // update new to old position map
            let old_pos = new_pos_to_old_pos[&intermediate_position];
            new_pos_to_old_pos.remove(&intermediate_position);
            new_pos_to_old_pos.insert(new_intermediate_position, old_pos);

            // update old to new position map
            *old_pos_to_new_pos.get_mut(&old_pos).unwrap() = new_intermediate_position;

            print_new_order(&input, &old_pos_to_new_pos);
        }
        // now update the nr in question
        *old_pos_to_new_pos.get_mut(&(prior_pos as i32)).unwrap() = updated_position;
        new_pos_to_old_pos.insert(updated_position, prior_pos as i32);
        print_new_order(&input, &old_pos_to_new_pos);
    }

    let mut new_vec: Vec<i32> = vec![0; input_len as usize];
    for i in 0..input.len() {
        let nr = input[i];
        let new_pos = old_pos_to_new_pos[&(i as i32)];
        new_vec[new_pos as usize] = nr;
    }

    println!("{:?}", new_vec);
    Ok(0)
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
