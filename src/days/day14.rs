use crossterm::{cursor, terminal, QueueableCommand};
use std::collections::HashMap;
use std::fs;
use std::io::{stdout, Write};
use std::{thread, time};

//const FILE_PATH: &str = "inputs/day14.txt";
const FILE_PATH: &str = "inputs/day14_test.txt";

fn read_input(file_path: &str) -> Vec<Vec<(i32, i32)>> {
    let file_contents: String = fs::read_to_string(file_path)
        .expect(format!("Could not read file '{}'", file_path).as_str());

    let mut output: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut inner_vector: Vec<(i32, i32)>;

    for line in file_contents.lines() {
        inner_vector = Vec::new();
        for substring in line.split_whitespace() {
            if substring == "->" {
                continue;
            }
            let coords: Vec<&str> = substring.split(',').collect();
            assert_eq!(coords.len(), 2);
            inner_vector.push((coords[0].parse().unwrap(), coords[1].parse().unwrap()));
        }
        output.push(inner_vector);
    }
    return output;
}

fn get_cave_slice_bounds(input: &Vec<Vec<(i32, i32)>>) -> (i32, i32, i32, i32) {
    let mut minx: i32 = std::i32::MAX;
    let mut maxx: i32 = std::i32::MIN;
    let mut miny: i32 = std::i32::MAX;
    let mut maxy: i32 = std::i32::MIN;

    for inner_vec in input.iter() {
        for coord in inner_vec {
            if coord.0 < minx {
                minx = coord.0;
            }
            if coord.0 > maxx {
                maxx = coord.0;
            }
            if coord.1 < miny {
                miny = coord.1;
            }
            if coord.1 > maxy {
                maxy = coord.1;
            }
        }
    }

    assert_eq!(miny > 0, true); // rock falls in from 500,0

    (minx, maxx, 0, maxy)
}

fn draw_cave(
    drawer: &mut Drawer,
    cave_matter: &HashMap<(i32, i32), Matter>,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
) -> () {
    let mut string: String = "".to_string();
    for y in miny..=maxy {
        string.push_str("\n");
        for x in minx..=maxx {
            if !cave_matter.contains_key(&(x, y)) {
                string.push_str(".");
            } else if cave_matter[&(x, y)] == Matter::Rock {
                string.push_str("#");
            } else if cave_matter[&(x, y)] == Matter::Sand {
                string.push_str("o");
            } else if cave_matter[&(x, y)] == Matter::Hole {
                string.push_str("+");
            }
        }
    }
    string.push_str("");
    drawer.draw_anew(string);
}

struct Drawer {
    stdout: std::io::Stdout,
}
impl Drawer {
    fn new() -> Drawer {
        let mut stdout: std::io::Stdout = stdout();
        stdout.queue(cursor::SavePosition).unwrap();
        Drawer { stdout }
    }

    fn set_position(&mut self) -> () {
        self.stdout.queue(cursor::SavePosition).unwrap();
    }

    fn draw_anew(&mut self, input: String) -> () {
        self.stdout.queue(cursor::RestorePosition).unwrap();
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();

        thread::sleep(time::Duration::from_millis(100));
        self.stdout
            .write_all(format!("{}: \nFOOBAR ", input).as_bytes())
            .unwrap();
        self.stdout.flush().unwrap();
    }
    fn wait_ms(&self, time_in_ms: i32) -> () {
        thread::sleep(time::Duration::from_millis(time_in_ms as u64));
    }
}

#[derive(PartialEq)]
enum Matter {
    Rock,
    Sand,
    Hole,
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = read_input(FILE_PATH);
    let mut drawer: Drawer = Drawer::new();
    let (minx, maxx, miny, maxy) = get_cave_slice_bounds(&input);
    let hole: (i32, i32) = (500, 0);
    let mut cave_matter: HashMap<(i32, i32), Matter> = HashMap::new();
    for line in input.iter() {
        for i_coord in 0..line.len() - 1 {
            let (mut start_x, mut start_y) = line[i_coord];
            let (mut stop_x, mut stop_y) = line[i_coord + 1];
            if start_y > stop_y {
                let tmp = start_y;
                start_y = stop_y;
                stop_y = tmp;
            }
            if start_x > stop_x {
                let tmp = start_x;
                start_x = stop_x;
                stop_x = tmp;
            }
            for x in start_x..=stop_x {
                for y in start_y..=stop_y {
                    cave_matter.insert((x, y), Matter::Rock);
                }
            }
            cave_matter.insert(hole, Matter::Hole);
        }
    }
    drawer.set_position();
    for i in 1..30 {
        drawer.draw_anew(format!("{} hello", i).to_string());
    }
    //return Ok(0);
    draw_cave(&mut drawer, &cave_matter, minx, maxx, miny, maxy);
    for i_sand_tile in 0..std::i32::MAX {
        let mut tile_pos: (i32, i32) = hole;
        for _ in 0..std::i32::MAX {
            if tile_pos.1 == maxy {
                return Ok(i_sand_tile);
            }
            if !cave_matter.contains_key(&(tile_pos.0, tile_pos.1 + 1)) {
                tile_pos = (tile_pos.0, tile_pos.1 + 1);
                continue;
            } else if !cave_matter.contains_key(&(tile_pos.0 - 1, tile_pos.1 + 1)) {
                tile_pos = (tile_pos.0 - 1, tile_pos.1 + 1);
                continue;
            } else if !cave_matter.contains_key(&(tile_pos.0 + 1, tile_pos.1 + 1)) {
                tile_pos = (tile_pos.0 + 1, tile_pos.1 + 1);
                continue;
            } else {
                cave_matter.insert(tile_pos, Matter::Sand);
                break;
            }
        }
        drawer.wait_ms(100);
        draw_cave(&mut drawer, &cave_matter, minx, maxx, miny, maxy);
    }
    Err("ran to the end")
}

pub fn result_b() -> Result<i32, &'static str> {
    let input = read_input(FILE_PATH);
    let (_minx, _maxx, _miny, mut maxy) = get_cave_slice_bounds(&input);
    maxy += 2;
    let hole: (i32, i32) = (500, 0);
    let mut cave_matter: HashMap<(i32, i32), Matter> = HashMap::new();
    for line in input.iter() {
        for i_coord in 0..line.len() - 1 {
            let (mut start_x, mut start_y) = line[i_coord];
            let (mut stop_x, mut stop_y) = line[i_coord + 1];
            if start_y > stop_y {
                let tmp = start_y;
                start_y = stop_y;
                stop_y = tmp;
            }
            if start_x > stop_x {
                let tmp = start_x;
                start_x = stop_x;
                stop_x = tmp;
            }
            for x in start_x..=stop_x {
                for y in start_y..=stop_y {
                    cave_matter.insert((x, y), Matter::Rock);
                }
            }
            cave_matter.insert(hole, Matter::Hole);
        }
    }
    // draw_cave(&cave_matter, minx, maxx, miny, maxy);
    for i_sand_tile in 1..std::i32::MAX {
        let mut tile_pos: (i32, i32) = hole;
        for _ in 0..std::i32::MAX {
            if tile_pos.1 == maxy - 1 {
                cave_matter.insert(tile_pos, Matter::Sand);
                break;
            }
            if !cave_matter.contains_key(&(tile_pos.0, tile_pos.1 + 1)) {
                tile_pos = (tile_pos.0, tile_pos.1 + 1);
                continue;
            } else if !cave_matter.contains_key(&(tile_pos.0 - 1, tile_pos.1 + 1)) {
                tile_pos = (tile_pos.0 - 1, tile_pos.1 + 1);
                continue;
            } else if !cave_matter.contains_key(&(tile_pos.0 + 1, tile_pos.1 + 1)) {
                tile_pos = (tile_pos.0 + 1, tile_pos.1 + 1);
                continue;
            } else if tile_pos == hole {
                return Ok(i_sand_tile);
            } else {
                cave_matter.insert(tile_pos, Matter::Sand);
                break;
            }
        }
        // if i_sand_tile % 1 == 0 {
        // draw_cave(&cave_matter, minx - 10, maxx + 10, miny, maxy);
        // }
    }
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
