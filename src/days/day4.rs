use crate::utils;

fn get_assignments() -> Vec<Vec<Vec<i32>>> {
    let file_path = "inputs/day4.txt";
    utils::file_path_plus_inner_sep_plus_outer_sep_to_vec_of_vec_of_vec_of_nr(file_path, "-", ",")
}

pub fn result_a() -> Result<i32, &'static str> {
    /*
     * Input is rows of "a-b,c-d" representing two assignments containing 2 numbers each.
     * The two numbers in an assignment span a range.
     * Find out how many ranges fully contains the other range, in an assignment pair.
     */

    let assignments: Vec<Vec<Vec<i32>>> = get_assignments();
    let mut nr_contained_assignments = 0;

    for assignment_pair in assignments.iter() {
        let pair_one = &assignment_pair[0];
        let pair_two = &assignment_pair[1];
        if (pair_one[0] <= pair_two[0] && pair_one[1] >= pair_two[1])
            || (pair_two[0] <= pair_one[0] && pair_two[1] >= pair_one[1])
        {
            nr_contained_assignments += 1;
        }
    }
    Ok(nr_contained_assignments)
}

pub fn result_b() -> Result<i32, &'static str> {
    /*
     * Input is rows of "a-b,c-d" representing two assignments containing 2 numbers each.
     * The two numbers in an assignment span a range.
     * Find out how many ranges overlap each other, in an assignment pair.
     * for example, 1-2,2,3 has an overlap (the 2)
     */

    let assignments: Vec<Vec<Vec<i32>>> = get_assignments();
    let mut nr_overlapping_assignments = 0;

    for assignment_pair in assignments.iter() {
        let pair_one = &assignment_pair[0];
        let pair_two = &assignment_pair[1];
        if pair_one[0] <= pair_two[1] && pair_one[1] >= pair_two[0] {
            nr_overlapping_assignments += 1;
        }
    }
    Ok(nr_overlapping_assignments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 513);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 878);
    }
}
