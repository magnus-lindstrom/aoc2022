use crate::utils;
use std::collections::{HashMap, HashSet};
const FILE_PATH: &str = "inputs/day15.txt";
const TEST_FILE_PATH: &str = "inputs/day15_test.txt";

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32
}

fn get_input(
    file_path: &str,
) -> (
    HashMap<(i32, i32), i32>,
    HashSet<(i32, i32)>,
    i32,
    i32,
    i32,
    i32,
) {
    let input = utils::vector_of_string_vectors_from_file(file_path);
    let (mut minx, mut maxx, mut miny, mut maxy) =
        (std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN);
    let mut sensors: HashMap<(i32, i32), i32> = HashMap::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();
    for line in input.iter() {
        let x_sensor: i32 = line[2]
            .trim_end_matches(',')
            .trim_start_matches(&['x', '='])
            .parse()
            .unwrap();
        let y_sensor: i32 = line[3]
            .trim_end_matches(':')
            .trim_start_matches(&['y', '='])
            .parse()
            .unwrap();
        if x_sensor < minx {
            minx = x_sensor;
        }
        if x_sensor > maxx {
            maxx = x_sensor;
        }
        if y_sensor < miny {
            miny = y_sensor;
        }
        if y_sensor > maxy {
            maxy = y_sensor;
        }

        let x_beacon: i32 = line[8]
            .trim_end_matches(',')
            .trim_start_matches(&['x', '='])
            .parse()
            .unwrap();
        let y_beacon: i32 = line[9].trim_start_matches(&['y', '=']).parse().unwrap();
        if x_beacon < minx {
            minx = x_beacon;
        }
        if x_beacon > maxx {
            maxx = x_beacon;
        }
        if y_beacon < miny {
            miny = y_beacon;
        }
        if y_beacon > maxy {
            maxy = y_beacon;
        }

        let dist_to_beacon = manhattan_dist((x_sensor, y_sensor), (x_beacon, y_beacon));

        sensors.insert((x_sensor, y_sensor), dist_to_beacon);
        beacons.insert((x_beacon, y_beacon));
    }
    (sensors, beacons, minx, maxx, miny, maxy)
}

/// not wrap around safe
fn in_safe_dist_of_any_sensor(point: (i32, i32), sensors: &HashMap<(i32, i32), i32>) -> bool {
    for sensor in sensors.keys().into_iter() {
        let dist_to_sensor = manhattan_dist(point, *sensor);
        if dist_to_sensor <= sensors[sensor] {
            return true;
        }
    }
    false
}

fn dist_to_closest_sensor(point: (i32, i32), sensors: &HashMap<(i32, i32), i32>) -> i32 {
    let mut closest: i32 = std::i32::MAX;
    for sensor in sensors.keys().into_iter() {
        let dist = manhattan_dist(point, *sensor);
        if dist < closest {
            closest = dist;
        }
    }
    closest
}

fn dist_to_closest_beacon_free_area(point: (i32, i32), sensors: &HashMap<(i32, i32), i32>) -> i32 {
    let mut closest: i32 = std::i32::MAX;
    for sensor in sensors.keys().into_iter() {
        let dist_to_sensor = manhattan_dist(point, *sensor);
        let dist_to_area = dist_to_sensor - sensors[sensor];
        if dist_to_area < closest {
            closest = dist_to_area;
            // println!("dist to sensor: {}", dist_to_sensor);
            // println!("dist to area: {}", dist_to_area);
            // println!("closest was: {}", closest);
        }
    }
    closest
}

fn all_sensors_are_behind(point: (i32, i32), sensors: &HashMap<(i32, i32), i32>) -> bool {
    for sensor in sensors.keys() {
        if sensor.0 >= point.0 {
            return false;
        }
    }
    true
}

pub fn result_a() -> Result<i32, &'static str> {
    let mode = "real";
    let file_path: &str;
    let y: i32;
    if mode == "real" {
        file_path = FILE_PATH;
        y = 10;
    } else if mode == "test" {
        file_path = TEST_FILE_PATH;
        y = 2000000;
    } else {
        panic!("invalid mode");
    }
    let (sensors, beacons, _minx, _maxx, _miny, _maxy) = get_input(file_path);
    let mut nr_spots_that_cant_be_beacon = 0;
    println!("sensors: {:?}", sensors);
    println!("beacons: {:?}", beacons);
    let mut x = std::i32::MIN + 1000;
    while x < std::i32::MAX {
        if in_safe_dist_of_any_sensor((x, y), &sensors) && !beacons.contains(&(x, y)) {
            // println!("no beacon at x,y: {},{}", x, y);
            nr_spots_that_cant_be_beacon += 1;
            x += 1;
        } else {
            if all_sensors_are_behind((x, y), &sensors) {
                println!("all are behind at {},{}", x, y);
                break;
            }
            //println!("possible beacon at x,y: {},{}", x, y);
            let dist_to_closest_beacon_free_area =
                dist_to_closest_beacon_free_area((x, y), &sensors);
            if dist_to_closest_beacon_free_area == 0 {
                x += 1
            } else {
                x += dist_to_closest_beacon_free_area;
            }
        }
    }
    Ok(nr_spots_that_cant_be_beacon)
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
