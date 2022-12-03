use std::collections::{HashMap, HashSet};
use std::fs;

fn get_priorities() -> HashMap<char, i32> {
    let mut item_priorities: HashMap<char, i32> = HashMap::new();
    // init prio here since the try_into complains when you add the result to an i32 directly
    let mut prio: i32;
    for (priority, item_type) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
    {
        prio = priority
            .try_into()
            .expect("Could not convert usize into i32");
        prio += 1;
        item_priorities.insert(item_type, prio);
    }
    return item_priorities;
}

pub fn result_a() -> Result<i32, &'static str> {
    /*
     * Each line is a list of items in a single rucksack.
     * Items are represented by chars
     * Each line can be split in two halves: one left and one right
     *
     * Find the single char that exists in both halfs
     */

    let file_path = "inputs/day3.txt";
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut total_priority: i32 = 0;

    let item_priorities = get_priorities();

    let mut half_of_line_len: usize;
    let mut item_types_in_left_compartment: HashSet<char>;
    for line in file_contents.lines() {
        half_of_line_len = line.len() / 2;
        item_types_in_left_compartment = HashSet::new();

        for item_type in line[..half_of_line_len].chars() {
            if !item_types_in_left_compartment.contains(&item_type) {
                item_types_in_left_compartment.insert(item_type);
            }
        }
        for item_type in line[half_of_line_len..].chars() {
            if item_types_in_left_compartment.contains(&item_type) {
                total_priority += item_priorities[&item_type];
                break;
            }
        }
    }

    Ok(total_priority)
}

pub fn result_b() -> Result<i32, &'static str> {
    /*
     * Same rucksacks as in a)
     * Every group of 3 consecutive rucksacks make a triplet
     *
     * Find the single char that exists in all three rucksacks
     */

    let file_path = "inputs/day3.txt";
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());
    let mut total_priority: i32 = 0;

    let item_priorities = get_priorities();

    let mut common_items_in_previous_rucksacks: HashSet<char> = HashSet::new();
    let mut common_items_in_all_rucksacks: HashSet<char> = HashSet::new();
    let mut group_member: i32 = 0;
    for rucksack in file_contents.lines() {
        if group_member == 0 {
            // all items belong in both sets
            for item_type in rucksack.chars() {
                common_items_in_previous_rucksacks.insert(item_type);
                common_items_in_all_rucksacks.insert(item_type);
            }
        } else {
            for item_type in rucksack.chars() {
                if common_items_in_previous_rucksacks.contains(&item_type) {
                    common_items_in_all_rucksacks.insert(item_type);
                }
            }
        }
        if group_member == 2 {
            assert_eq!(common_items_in_all_rucksacks.len(), 1);
            for item_type in &common_items_in_all_rucksacks {
                total_priority += item_priorities[&item_type];
            }
        }

        if group_member == 2 {
            group_member = 0;
            common_items_in_previous_rucksacks.clear();
        } else {
            group_member += 1;
        }
        common_items_in_previous_rucksacks.clear();
        common_items_in_previous_rucksacks.extend(&common_items_in_all_rucksacks);
        common_items_in_all_rucksacks.clear();
    }

    Ok(total_priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a();
        assert_eq!(answer, Ok(7793));
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b();
        assert_eq!(answer, Ok(2499));
    }
}
