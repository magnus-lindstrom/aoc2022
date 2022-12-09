use crate::utils;
use std::collections::HashSet;

const FILE_PATH: &str = "inputs/day9.txt";

fn move_head(head_pos: &mut (i32, i32), move_dir: char) -> (i32, i32) {
    match move_dir {
        'U' => head_pos.1 += 1,
        'D' => head_pos.1 -= 1,
        'L' => head_pos.0 -= 1,
        'R' => head_pos.0 += 1,
        _ => panic!("Got unexpected direction"),
    }
    *head_pos
}

fn new_tail_pos(head_pos: &(i32, i32), tail_pos: &(i32, i32)) -> (i32, i32) {
    let pos_diffs: (i32, i32) = ((head_pos.0 - tail_pos.0), (head_pos.1 - tail_pos.1));
    let mut to_move: (i32, i32) = (0, 0);
    if pos_diffs.0.abs() > 1 {
        if pos_diffs.1 != 0 {
            to_move = (pos_diffs.0.signum(), pos_diffs.1.signum());
        } else {
            to_move = (pos_diffs.0.signum(), 0);
        }
    } else if pos_diffs.1.abs() > 1 {
        if pos_diffs.0 != 0 {
            to_move = (pos_diffs.0.signum(), pos_diffs.1.signum());
        } else {
            to_move = (0, pos_diffs.1.signum());
        }
    }

    let new_pos: (i32, i32) = (tail_pos.0 + to_move.0, tail_pos.1 + to_move.1);
    new_pos
}

fn solve_for_nr_knots(nr_knots: usize) -> Result<usize, &'static str> {
    let rope_moves: Vec<(char, i32)> = utils::file_path_to_vec_of_char_nr_tuples(FILE_PATH);
    let mut have_been: HashSet<(i32, i32)> = HashSet::new();
    have_been.insert((0, 0));
    // element 0 is the head
    let mut knot_positions: Vec<(i32, i32)> = vec![(0, 0); nr_knots];
    // println!("{:?}", knot_positions);
    // println!("{:?}", knot_positions.len());

    for rope_move in rope_moves.iter() {
        let move_dir = rope_move.0;
        let move_repeats = rope_move.1;
        for _ in 0..move_repeats {
            knot_positions[0] = move_head(&mut knot_positions[0], move_dir);
            for i in 1..nr_knots {
                knot_positions[i] = new_tail_pos(&knot_positions[i - 1], &knot_positions[i]);
            }
            // println!("{:?}", knot_positions);
            have_been.insert(*knot_positions.last().unwrap());
        }
    }
    Ok(have_been.len())
}

/// Input is a list of moves of a "rope head".
/// The "rope tail" will always take a step towards the head.
/// Find out the number of unique positions that have been occupied by the tail.
pub fn result_a() -> Result<usize, &'static str> {
    solve_for_nr_knots(2)
}

pub fn result_b() -> Result<usize, &'static str> {
    solve_for_nr_knots(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 6332);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 2511);
    }
}
