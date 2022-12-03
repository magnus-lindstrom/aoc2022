use crate::utils;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum RPSResult {
    Win,
    Loss,
    Draw,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

fn get_scores() -> (HashMap<RPSMove, i32>, HashMap<RPSResult, i32>) {
    let move_scores: HashMap<RPSMove, i32> = HashMap::from([
        (RPSMove::Rock, 1),
        (RPSMove::Paper, 2),
        (RPSMove::Scissors, 3),
    ]);
    let matchup_scores: HashMap<RPSResult, i32> = HashMap::from([
        (RPSResult::Win, 6),
        (RPSResult::Loss, 0),
        (RPSResult::Draw, 3),
    ]);
    (move_scores, matchup_scores)
}

pub fn result_a() -> Result<i32, &'static str> {
    let file_path = "inputs/day2.txt";
    let all_rps_games = utils::vector_of_string_vectors_from_file(file_path);
    let (move_scores, matchup_scores) = get_scores();

    let move_dict: HashMap<String, RPSMove> = HashMap::from([
        ("A".to_string(), RPSMove::Rock),
        ("X".to_string(), RPSMove::Rock),
        ("B".to_string(), RPSMove::Paper),
        ("Y".to_string(), RPSMove::Paper),
        ("C".to_string(), RPSMove::Scissors),
        ("Z".to_string(), RPSMove::Scissors),
    ]);

    let mut my_move: RPSMove;
    let mut their_move: RPSMove;
    let mut my_score = 0;
    for rps_game in all_rps_games {
        my_move = move_dict[&rps_game[1]];
        their_move = move_dict[&rps_game[0]];

        // add matchup score
        match my_move {
            RPSMove::Rock => match their_move {
                RPSMove::Rock => my_score += matchup_scores[&RPSResult::Draw],
                RPSMove::Paper => my_score += matchup_scores[&RPSResult::Loss],
                RPSMove::Scissors => my_score += matchup_scores[&RPSResult::Win],
            },
            RPSMove::Paper => match their_move {
                RPSMove::Rock => my_score += matchup_scores[&RPSResult::Win],
                RPSMove::Paper => my_score += matchup_scores[&RPSResult::Draw],
                RPSMove::Scissors => my_score += matchup_scores[&RPSResult::Loss],
            },
            RPSMove::Scissors => match their_move {
                RPSMove::Rock => my_score += matchup_scores[&RPSResult::Loss],
                RPSMove::Paper => my_score += matchup_scores[&RPSResult::Win],
                RPSMove::Scissors => my_score += matchup_scores[&RPSResult::Draw],
            },
        }

        // add move score
        my_score += move_scores[&my_move];
    }
    Ok(my_score)
}

pub fn result_b() -> Result<i32, &'static str> {
    let file_path = "inputs/day2.txt";
    let all_rps_games = utils::vector_of_string_vectors_from_file(file_path);
    let (move_scores, matchup_scores) = get_scores();

    let their_move_dict: HashMap<String, RPSMove> = HashMap::from([
        ("A".to_string(), RPSMove::Rock),
        ("B".to_string(), RPSMove::Paper),
        ("C".to_string(), RPSMove::Scissors),
    ]);

    let intended_result_dict: HashMap<String, RPSResult> = HashMap::from([
        ("X".to_string(), RPSResult::Loss),
        ("Y".to_string(), RPSResult::Draw),
        ("Z".to_string(), RPSResult::Win),
    ]);

    let mut intended_result: RPSResult;
    let mut my_move: RPSMove;
    let mut their_move: RPSMove;
    let mut my_score = 0;
    for rps_game in all_rps_games.into_iter() {
        their_move = their_move_dict[&rps_game[0]];
        intended_result = intended_result_dict[&rps_game[1]];

        // get my move
        match intended_result {
            RPSResult::Win => match their_move {
                RPSMove::Rock => my_move = RPSMove::Paper,
                RPSMove::Paper => my_move = RPSMove::Scissors,
                RPSMove::Scissors => my_move = RPSMove::Rock,
            },
            RPSResult::Draw => {
                my_move = their_move;
            }
            RPSResult::Loss => match their_move {
                RPSMove::Rock => my_move = RPSMove::Scissors,
                RPSMove::Paper => my_move = RPSMove::Rock,
                RPSMove::Scissors => my_move = RPSMove::Paper,
            },
        }

        // add matchup score
        match my_move {
            RPSMove::Rock => match their_move {
                RPSMove::Rock => my_score += matchup_scores[&RPSResult::Draw],
                RPSMove::Paper => my_score += matchup_scores[&RPSResult::Loss],
                RPSMove::Scissors => my_score += matchup_scores[&RPSResult::Win],
            },
            RPSMove::Paper => match their_move {
                RPSMove::Rock => my_score += matchup_scores[&RPSResult::Win],
                RPSMove::Paper => my_score += matchup_scores[&RPSResult::Draw],
                RPSMove::Scissors => my_score += matchup_scores[&RPSResult::Loss],
            },
            RPSMove::Scissors => match their_move {
                RPSMove::Rock => my_score += matchup_scores[&RPSResult::Loss],
                RPSMove::Paper => my_score += matchup_scores[&RPSResult::Win],
                RPSMove::Scissors => my_score += matchup_scores[&RPSResult::Draw],
            },
        }

        // add move score
        my_score += move_scores[&my_move];
    }
    Ok(my_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a();
        assert_eq!(answer, Ok(14375));
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b();
        assert_eq!(answer, Ok(10274));
    }
}
