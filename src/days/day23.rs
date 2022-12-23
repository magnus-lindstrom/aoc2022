use std::collections::{HashMap, HashSet};

const FILE_PATH: &str = "inputs/day23.txt";
//const FILE_PATH: &str = "inputs/day23_test.txt";

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Boundaries {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}
impl Boundaries {
    fn from_map(map: &HashSet<(i32, i32)>) -> Boundaries {
        let mut minx = std::i32::MAX;
        let mut miny = std::i32::MAX;
        let mut maxx = std::i32::MIN;
        let mut maxy = std::i32::MIN;
        for point in map.iter() {
            if point.0 < minx {
                minx = point.0 as i32;
            } else if point.0 > maxx {
                maxx = point.0 as i32;
            }
            if point.1 < miny {
                miny = point.1;
            } else if point.1 > maxy {
                maxy = point.1;
            }
        }
        Boundaries {
            minx,
            miny,
            maxx,
            maxy,
        }
    }

    fn get_empty_tiles(&self, map: &HashSet<(i32, i32)>) -> i32 {
        let mut empty_tiles = 0;
        for i_row in self.minx..=self.maxx {
            for j_col in self.miny..=self.maxy {
                if !map.contains(&(i_row, j_col)) {
                    empty_tiles += 1;
                }
            }
        }
        empty_tiles
    }
}

#[allow(dead_code)]
fn draw_map(map: &HashSet<(i32, i32)>) -> () {
    let mut string: String = "".to_string();
    let b = Boundaries::from_map(map);
    for i_row in b.minx..=b.maxx {
        for j_col in b.miny..=b.maxy {
            if map.contains(&(i_row, j_col)) {
                string.push('#');
            } else {
                string.push('.');
            }
        }
        string.push('\n');
    }
    println!("{}", string);
}

fn no_adjacent_elf(elf: (i32, i32), map_of_elves: &HashSet<(i32, i32)>) -> bool {
    if map_of_elves.contains(&(elf.0 - 1, elf.1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0 - 1, elf.1 + 1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0, elf.1 + 1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0 + 1, elf.1 + 1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0 + 1, elf.1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0 + 1, elf.1 - 1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0, elf.1 - 1)) {
        return false;
    } else if map_of_elves.contains(&(elf.0 - 1, elf.1 - 1)) {
        return false;
    }
    true
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
    let mut map_of_elves: HashSet<(i32, i32)> = HashSet::new();
    let mut proposed_change_to_elves: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut move_dirs: Vec<Direction> = Vec::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for (i_row, line) in input.iter().enumerate() {
        for (j_col, ch) in line.chars().enumerate() {
            if ch == '#' {
                map_of_elves.insert((i_row as i32, j_col as i32));
            }
        }
    }
    //draw_map(&map_of_elves);

    for _ in 0..10 {
        proposed_change_to_elves.clear();
        for elf in map_of_elves.iter() {
            if no_adjacent_elf(*elf, &map_of_elves) {
                continue;
            }
            let mut has_proposed = false;
            for move_dir in move_dirs.iter() {
                if *move_dir == Direction::North {
                    if !map_of_elves.contains(&(elf.0 - 1, elf.1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 + 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0 - 1, elf.1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0 - 1, elf.1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0 - 1, elf.1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                } else if *move_dir == Direction::South {
                    if !map_of_elves.contains(&(elf.0 + 1, elf.1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0 + 1, elf.1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0 + 1, elf.1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0 + 1, elf.1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                } else if *move_dir == Direction::West {
                    if !map_of_elves.contains(&(elf.0, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 - 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0, elf.1 - 1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0, elf.1 - 1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0, elf.1 - 1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                } else if *move_dir == Direction::East {
                    if !map_of_elves.contains(&(elf.0, elf.1 + 1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0, elf.1 + 1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0, elf.1 + 1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0, elf.1 + 1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                }
                if has_proposed {
                    break;
                }
            }
        }

        for change in proposed_change_to_elves.keys() {
            if proposed_change_to_elves[change].len() == 1 {
                ////draw_map(&map_of_elves);
                map_of_elves.insert(*change);
                map_of_elves.remove(&proposed_change_to_elves[change][0]);
            }
        }
        let first_dir = move_dirs.remove(0);
        move_dirs.push(first_dir);
        //draw_map(&map_of_elves);
    }
    let b: Boundaries = Boundaries::from_map(&map_of_elves);
    let empty_tiles_in_smallest_rectangle = b.get_empty_tiles(&map_of_elves);
    Ok(empty_tiles_in_smallest_rectangle)
}

pub fn result_b() -> Result<i32, &'static str> {
    let input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
    let mut map_of_elves: HashSet<(i32, i32)> = HashSet::new();
    let mut proposed_change_to_elves: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut move_dirs: Vec<Direction> = Vec::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for (i_row, line) in input.iter().enumerate() {
        for (j_col, ch) in line.chars().enumerate() {
            if ch == '#' {
                map_of_elves.insert((i_row as i32, j_col as i32));
            }
        }
    }
    //draw_map(&map_of_elves);

    let mut someone_has_moved = true;
    let mut round = 0;
    while someone_has_moved {
        round += 1;
        proposed_change_to_elves.clear();
        for elf in map_of_elves.iter() {
            if no_adjacent_elf(*elf, &map_of_elves) {
                continue;
            }
            let mut has_proposed = false;
            for move_dir in move_dirs.iter() {
                if *move_dir == Direction::North {
                    if !map_of_elves.contains(&(elf.0 - 1, elf.1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 + 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0 - 1, elf.1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0 - 1, elf.1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0 - 1, elf.1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                } else if *move_dir == Direction::South {
                    if !map_of_elves.contains(&(elf.0 + 1, elf.1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0 + 1, elf.1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0 + 1, elf.1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0 + 1, elf.1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                } else if *move_dir == Direction::West {
                    if !map_of_elves.contains(&(elf.0, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 - 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0, elf.1 - 1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0, elf.1 - 1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0, elf.1 - 1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                } else if *move_dir == Direction::East {
                    if !map_of_elves.contains(&(elf.0, elf.1 + 1))
                        && !map_of_elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !map_of_elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        if proposed_change_to_elves.contains_key(&(elf.0, elf.1 + 1)) {
                            proposed_change_to_elves
                                .get_mut(&(elf.0, elf.1 + 1))
                                .unwrap()
                                .push(*elf);
                        } else {
                            proposed_change_to_elves.insert((elf.0, elf.1 + 1), vec![*elf]);
                        }
                        has_proposed = true;
                    }
                }
                if has_proposed {
                    break;
                }
            }
        }

        someone_has_moved = false;
        for change in proposed_change_to_elves.keys() {
            if proposed_change_to_elves[change].len() == 1 {
                ////draw_map(&map_of_elves);
                map_of_elves.insert(*change);
                map_of_elves.remove(&proposed_change_to_elves[change][0]);
                someone_has_moved = true;
            }
        }
        let first_dir = move_dirs.remove(0);
        move_dirs.push(first_dir);
        //draw_map(&map_of_elves);
    }
    Ok(round)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 4254);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 992);
    }
}
