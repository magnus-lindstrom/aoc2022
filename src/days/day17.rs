use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Write;
//const FILE_PATH: &str = "inputs/day17_test.txt";
const FILE_PATH: &str = "inputs/day17.txt";

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Matter {
    FallingRock,
    RestingRock,
    Wall,
    Floor,
    Corner,
    Air,
}

fn get_matter_representations() -> HashMap<Matter, char> {
    let mut matter_repr = HashMap::new();

    matter_repr.insert(Matter::FallingRock, '@');
    matter_repr.insert(Matter::RestingRock, '#');
    matter_repr.insert(Matter::Floor, '_');
    matter_repr.insert(Matter::Wall, '|');
    matter_repr.insert(Matter::Corner, '+');
    matter_repr.insert(Matter::Air, '.');

    matter_repr
}

#[allow(dead_code)]
fn draw_room_to_file(room: &Vec<Vec<Matter>>) -> () {
    let matter_repr = get_matter_representations();
    let mut string: String = "".to_string();
    for level in room.iter().rev() {
        for matter in level.iter() {
            string.push(matter_repr[matter]);
        }
        string.push('\n');
    }
    string.push('\n');
    let mut file = std::fs::File::create("tmpout").unwrap();
    file.write(string.as_bytes()).unwrap();
}

#[allow(dead_code)]
fn draw_room(room: &Vec<Vec<Matter>>) -> () {
    let matter_repr = get_matter_representations();
    let mut string: String = "".to_string();
    for level in room.iter().rev() {
        for matter in level.iter() {
            string.push(matter_repr[matter]);
        }
        string.push('\n');
    }
    string.push('\n');
    for (i, line) in string.lines().enumerate() {
        println!("{}", line);
        if i == 20 {
            println!("--------------\n");
            break;
        }
    }
    //crate::utils::draw_and_sleep_ms(string.as_str(), 50);
}

fn get_new_empty_level() -> Vec<Matter> {
    let mut level = Vec::new();
    level.push(Matter::Wall);
    for _ in 0..7 {
        level.push(Matter::Air);
    }
    level.push(Matter::Wall);
    level
}

fn get_new_level_with_rocks_at_indeces(rock_indeces: Vec<usize>) -> Vec<Matter> {
    let mut level = Vec::new();
    level.push(Matter::Wall);
    for _ in 0..7 {
        level.push(Matter::Air);
    }
    level.push(Matter::Wall);
    for rock_index in rock_indeces {
        level[rock_index + 1] = Matter::FallingRock;
    }
    level
}

fn get_initial_room(chamber_width: i32) -> Vec<Vec<Matter>> {
    let mut matter_vector: Vec<Vec<Matter>> = Vec::new();
    let mut floor: Vec<Matter> = Vec::new();
    floor.push(Matter::Corner);
    for _ in 0..chamber_width {
        floor.push(Matter::Floor);
    }
    floor.push(Matter::Corner);
    matter_vector.push(floor);
    matter_vector
}

fn add_rock_type_to_room(room: &mut Vec<Vec<Matter>>, rock_type: usize) -> () {
    room.push(get_new_empty_level());
    room.push(get_new_empty_level());
    room.push(get_new_empty_level());
    if rock_type == 0 {
        room.push(get_new_level_with_rocks_at_indeces(vec![2, 3, 4, 5]));
    } else if rock_type == 1 {
        room.push(get_new_level_with_rocks_at_indeces(vec![3]));
        room.push(get_new_level_with_rocks_at_indeces(vec![2, 3, 4]));
        room.push(get_new_level_with_rocks_at_indeces(vec![3]));
    } else if rock_type == 2 {
        room.push(get_new_level_with_rocks_at_indeces(vec![2, 3, 4]));
        room.push(get_new_level_with_rocks_at_indeces(vec![4]));
        room.push(get_new_level_with_rocks_at_indeces(vec![4]));
    } else if rock_type == 3 {
        room.push(get_new_level_with_rocks_at_indeces(vec![2]));
        room.push(get_new_level_with_rocks_at_indeces(vec![2]));
        room.push(get_new_level_with_rocks_at_indeces(vec![2]));
        room.push(get_new_level_with_rocks_at_indeces(vec![2]));
    } else if rock_type == 4 {
        room.push(get_new_level_with_rocks_at_indeces(vec![2, 3]));
        room.push(get_new_level_with_rocks_at_indeces(vec![2, 3]));
    } else {
        panic!("invalid rock type");
    }
}

fn hash_top_of_room(room: &Vec<Vec<Matter>>) -> u64 {
    let mut new_vec: Vec<Matter> = Vec::new();
    for i_level in (room.len() - 80..room.len()).rev() {
        for i_matter in 1..=7 {
            new_vec.push(room[i_level][i_matter]);
        }
    }
    let mut s = DefaultHasher::new();
    new_vec.hash(&mut s);
    s.finish()
}

fn blow_right(room: &mut Vec<Vec<Matter>>) -> () {
    let mut has_encountered_the_falling_rock = false;
    // go from top to bottom
    for i_level in (0..room.len()).rev() {
        let mut level_has_falling_rock = false;
        let level = &room[i_level];

        let mut falling_rock_to_the_left = false;
        // go from left to right over the level, see if a solid object is in the way of blowing to
        // the left
        for i_matter in 0..9 {
            if level[i_matter] == Matter::Wall || level[i_matter] == Matter::RestingRock {
                if falling_rock_to_the_left {
                    return; // can not blow right
                }
                falling_rock_to_the_left = false;
            } else if level[i_matter] == Matter::FallingRock {
                falling_rock_to_the_left = true;
                level_has_falling_rock = true;
                has_encountered_the_falling_rock = true;
            } else {
                falling_rock_to_the_left = false;
            }
        }
        if has_encountered_the_falling_rock && !level_has_falling_rock {
            break;
        }
    }

    // do the blowing
    let mut has_encountered_the_falling_rock = false;
    // go from top to bottom
    for i_level in (0..room.len()).rev() {
        let mut level_has_falling_rock = false;

        let mut new_level = room[i_level].clone();
        for i_matter in (2..=7).rev() {
            // println!("i_matter: {}, new level: {:?}", i_matter, new_level);
            if new_level[i_matter - 1] == Matter::FallingRock {
                new_level[i_matter] = Matter::FallingRock;
                new_level[i_matter - 1] = Matter::Air;
                has_encountered_the_falling_rock = true;
                level_has_falling_rock = true;
            }
        }
        room[i_level] = new_level;

        if has_encountered_the_falling_rock && !level_has_falling_rock {
            break;
        }
    }
}

fn blow_left(room: &mut Vec<Vec<Matter>>) -> () {
    let mut has_encountered_the_falling_rock = false;
    // first check if possible to blow left
    for i_level in (0..room.len()).rev() {
        let mut level_has_falling_rock = false;
        let level = &room[i_level];

        let mut solid_object_to_the_left = false;
        // go from left to right over the level, see if a solid object is in the way of blowing to
        // the left
        for i_matter in 0..9 {
            if level[i_matter] == Matter::Wall || level[i_matter] == Matter::RestingRock {
                solid_object_to_the_left = true;
            } else if level[i_matter] == Matter::FallingRock {
                if solid_object_to_the_left {
                    return;
                }
                solid_object_to_the_left = false;
                has_encountered_the_falling_rock = true;
                level_has_falling_rock = true;
            } else {
                solid_object_to_the_left = false;
            }
        }
        if has_encountered_the_falling_rock && !level_has_falling_rock {
            break;
        }
    }

    let mut has_encountered_the_falling_rock = false;
    // do the blowing
    for i_level in (0..room.len()).rev() {
        let mut level_has_falling_rock = false;
        let mut new_level = room[i_level].clone();
        for i_matter in 1..=6 {
            if new_level[i_matter + 1] == Matter::FallingRock {
                new_level[i_matter] = Matter::FallingRock;
                new_level[i_matter + 1] = Matter::Air;
                level_has_falling_rock = true;
                has_encountered_the_falling_rock = true;
            }
        }
        room[i_level] = new_level;
        if has_encountered_the_falling_rock && !level_has_falling_rock {
            break;
        }
    }
}

fn get_rock_level(room: &Vec<Vec<Matter>>) -> usize {
    for i_level in (0..room.len()).rev() {
        for i_matter in 1..=7 {
            if room[i_level][i_matter] == Matter::FallingRock {
                return i_level;
            }
        }
    }
    panic!("Could not find rock");
}

fn lower_rock_and_return_if_bottom_was_hit(room: &mut Vec<Vec<Matter>>) -> bool {
    // first find out if we can lower rock
    let mut falling_rock_above: Vec<bool> = vec![false; 7];
    let mut rock_has_been_seen = false;
    for i_level in (0..room.len()).rev() {
        let mut level_has_falling_rock = false;
        for i_matter in 1..=7 {
            if falling_rock_above[i_matter - 1]
                && (room[i_level][i_matter] == Matter::Floor
                    || room[i_level][i_matter] == Matter::RestingRock)
            {
                return true;
            }

            if room[i_level][i_matter] == Matter::FallingRock {
                falling_rock_above[i_matter - 1] = true;
                rock_has_been_seen = true;
                level_has_falling_rock = true;
            } else {
                falling_rock_above[i_matter - 1] = false;
            }
        }
        if rock_has_been_seen && !level_has_falling_rock {
            continue;
        }
    }

    let mut air_below = vec![false; 7];
    // do the lowering of the rock
    let mut start_level = get_rock_level(&room);
    if start_level < 5 {
        start_level = 0;
    } else {
        start_level -= 5;
    }
    for i_level in start_level..room.len() {
        // the bottom and upwards
        for i_matter in 1..=7 {
            if room[i_level][i_matter] == Matter::FallingRock {
                // air_below stays unchanged (if there is air below, the rock will fall, and this
                // level will also have air below for the next level)
                // if there isn't air below, there won't be for the next level either, since this
                // is rock
                if air_below[i_matter - 1] {
                    room[i_level][i_matter] = Matter::Air;
                    room[i_level - 1][i_matter] = Matter::FallingRock;
                }
            } else if room[i_level][i_matter] == Matter::Air {
                air_below[i_matter - 1] = true;
            } else if room[i_level][i_matter] == Matter::RestingRock
                || room[i_level][i_matter] == Matter::Floor
            {
                air_below[i_matter - 1] = true;
            }
        }
    }

    // remove upper level of air, if there is one
    let mut all_air: bool = true;
    for i_matter in 1..=7 {
        if room.last().unwrap()[i_matter] != Matter::Air {
            all_air = false;
        }
    }
    if all_air {
        room.pop();
    }

    false // did not hit rock bottom
}

fn convert_falling_rocks_to_resting(room: &mut Vec<Vec<Matter>>) -> () {
    let mut has_encountered_the_falling_rock: bool = false;
    for i_level in (0..room.len()).rev() {
        let mut level_had_falling_rock = false;
        for i_matter in 1..=7 {
            if room[i_level][i_matter] == Matter::FallingRock {
                room[i_level][i_matter] = Matter::RestingRock;
                has_encountered_the_falling_rock = true;
                level_had_falling_rock = true;
            }
        }
        if has_encountered_the_falling_rock && !level_had_falling_rock {
            return;
        }
    }
}

#[allow(dead_code)]
fn top_level_is_flat(room: &Vec<Vec<Matter>>) -> bool {
    for i_matter in 1..=7 {
        if room.last().unwrap()[i_matter] != Matter::RestingRock {
            // println!("{:?}", room.last().unwrap());
            return false;
        }
    }
    true
}

fn solver(total_rocks_to_drop: u64) -> Result<u64, &'static str> {
    let input = std::fs::read_to_string(FILE_PATH).unwrap();
    let winds: Vec<char> = input.trim().chars().collect();
    let total_rocks_to_drop = total_rocks_to_drop;
    let chamber_width = 7;
    let mut have_warped = false;
    let mut warped_height: u64 = 0;

    let mut room: Vec<Vec<Matter>> = get_initial_room(chamber_width);
    let mut rocks_dropped: u64 = 0;
    let mut rock_type_to_drop_next: usize = 0;
    let mut wind_to_blow_next: usize = 0;
    // structure + wind index + rock type, maps to rocks dropped and height of room
    let mut scenarios_seen: HashMap<(u64, usize, usize), (u64, u64)> = HashMap::new();
    while rocks_dropped < total_rocks_to_drop {
        let mut rock_has_hit_bottom = false;
        add_rock_type_to_room(&mut room, rock_type_to_drop_next);
        while !rock_has_hit_bottom {
            if winds[wind_to_blow_next] == '<' {
                blow_left(&mut room);
            } else if winds[wind_to_blow_next] == '>' {
                blow_right(&mut room);
            } else {
                panic!("Strange wind");
            }
            wind_to_blow_next = (wind_to_blow_next + 1) % winds.len();

            rock_has_hit_bottom = lower_rock_and_return_if_bottom_was_hit(&mut room);
            if rock_has_hit_bottom {
                convert_falling_rocks_to_resting(&mut room);
            }
        }
        rock_type_to_drop_next = (rock_type_to_drop_next + 1) % 5;
        rocks_dropped += 1;

        if !have_warped && room.len() > 100 {
            let top_hash = hash_top_of_room(&room);
            let structure_plus_wind_and_rock =
                (top_hash, wind_to_blow_next, rock_type_to_drop_next);
            if scenarios_seen.contains_key(&structure_plus_wind_and_rock) {
                let delta_rocks = rocks_dropped - scenarios_seen[&structure_plus_wind_and_rock].0;
                let delta_height =
                    room.len() as u64 - scenarios_seen[&structure_plus_wind_and_rock].1;
                loop {
                    if rocks_dropped + delta_rocks > total_rocks_to_drop {
                        break;
                    }
                    rocks_dropped += delta_rocks;
                    warped_height += delta_height;
                    have_warped = true;
                }
            } else {
                scenarios_seen.insert(
                    structure_plus_wind_and_rock,
                    (rocks_dropped, room.len() as u64),
                );
            }
        }
    }
    Ok(room.len() as u64 - 1 + warped_height)
}

// 2023 for answer
pub fn result_a() -> Result<u64, &'static str> {
    solver(2022)
}

pub fn result_b() -> Result<u64, &'static str> {
    solver(1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 3184);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 1577077363915);
    }
}
