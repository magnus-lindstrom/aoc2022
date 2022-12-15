use crate::utils;
use std::collections::{HashMap, HashSet};
const FILE_PATH: &str = "inputs/day15.txt";
const TEST_FILE_PATH: &str = "inputs/day15_test.txt";

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> u32 {
    /*
    println!(
    "dist between {},{} and {},{} is {}",
    a.0,
    a.1,
    b.0,
    b.1,
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    );
    */
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn get_input(file_path: &str) -> (HashMap<(i32, i32), u32>, HashSet<(i32, i32)>) {
    let input = utils::vector_of_string_vectors_from_file(file_path);
    let mut sensors: HashMap<(i32, i32), u32> = HashMap::new();
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
        let x_beacon: i32 = line[8]
            .trim_end_matches(',')
            .trim_start_matches(&['x', '='])
            .parse()
            .unwrap();
        let y_beacon: i32 = line[9].trim_start_matches(&['y', '=']).parse().unwrap();

        let dist_to_beacon = manhattan_dist((x_sensor, y_sensor), (x_beacon, y_beacon));

        sensors.insert((x_sensor, y_sensor), dist_to_beacon);
        beacons.insert((x_beacon, y_beacon));
    }
    (sensors, beacons)
}

fn in_safe_dist_of_any_sensor(point: (i32, i32), sensors: &HashMap<(i32, i32), u32>) -> bool {
    for sensor in sensors.keys().into_iter() {
        let dist_to_sensor = manhattan_dist(point, *sensor);
        if dist_to_sensor <= sensors[sensor] {
            return true;
        }
    }
    false
}

/// returns negative number if point is not inside sensor area
fn max_dist_to_end_of_sensor_area(point: (i32, i32), sensors: &HashMap<(i32, i32), u32>) -> i32 {
    let mut max_dist = std::i32::MIN;
    for sensor in sensors.keys().into_iter() {
        let dist_to_sensor = manhattan_dist(point, *sensor);
        let sensor_to_beacon_dist = sensors[sensor];
        let dist_diff = sensor_to_beacon_dist - dist_to_sensor;
        // println!("sensor is at {},{}", sensor.0, sensor.1);
        // println!("beacon is {} dist away", sensors[sensor]);
        // println!("point is {} dist away from sensor", dist_to_sensor);
        // println!("should be {} from sensor area edge", dist_diff);
        if dist_diff as i32 > max_dist {
            max_dist = dist_diff as i32;
        }
    }
    // println!("returning {}", max_dist);
    max_dist
}

/*
fn dist_to_closest_sensor(point: (i32, i32), sensors: &HashMap<(i32, i32), i32>) -> u32 {
let mut closest: u32 = std::u32::MAX;
for sensor in sensors.keys().into_iter() {
let dist = manhattan_dist(point, *sensor);
if dist < closest {
closest = dist;
}
}
closest
}
*/

fn dist_to_closest_beacon_free_area(point: (i32, i32), sensors: &HashMap<(i32, i32), u32>) -> u32 {
    let mut closest: u32 = std::u32::MAX;
    for sensor in sensors.keys().into_iter() {
        let dist_to_sensor = manhattan_dist(point, *sensor);
        let dist_to_area = dist_to_sensor - sensors[sensor];
        if dist_to_area < closest {
            // println!("closest was {}", closest);
            // println!("closest is now {}", dist_to_area);
            closest = dist_to_area;
            // println!("dist to sensor: {}", dist_to_sensor);
            // println!("dist to area: {}", dist_to_area);
            // println!("closest was: {}", closest);
        }
    }
    closest
}

fn all_sensors_are_behind(point: (i32, i32), sensors: &HashMap<(i32, i32), u32>) -> bool {
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
    if mode == "test" {
        file_path = TEST_FILE_PATH;
        y = 10;
    } else if mode == "real" {
        file_path = FILE_PATH;
        y = 2000000;
    } else {
        panic!("invalid mode");
    }
    let (sensors, beacons) = get_input(file_path);
    let mut nr_spots_that_cant_be_beacon = 0;
    let mut x = std::i32::MIN;
    while x < std::i32::MAX {
        // println!("x: {}", x);
        if in_safe_dist_of_any_sensor((x, y), &sensors) && !beacons.contains(&(x, y)) {
            // println!("no beacon at x,y: {},{}", x, y);
            nr_spots_that_cant_be_beacon += 1;
            x += 1;
        } else {
            if all_sensors_are_behind((x, y), &sensors) {
                // println!("all are behind at {},{}", x, y);
                break;
            }
            //println!("possible beacon at x,y: {},{}", x, y);
            let mut dist_to_closest_beacon_free_area =
                dist_to_closest_beacon_free_area((x, y), &sensors);
            if dist_to_closest_beacon_free_area == 0 {
                x += 1
            } else {
                if dist_to_closest_beacon_free_area > std::i32::MAX as u32 {
                    dist_to_closest_beacon_free_area = std::i32::MAX as u32;
                }
                //println!("x: {}", x);
                // println!("dist: {}", dist_to_closest_beacon_free_area);
                //panic!("overflow");
                x += dist_to_closest_beacon_free_area as i32;
            }
        }
    }
    Ok(nr_spots_that_cant_be_beacon)
}

pub fn result_b() -> Result<i64, &'static str> {
    let mode = "real";
    let file_path: &str;
    let xmin: i32;
    let ymin: i32;
    let xmax: i32;
    let ymax: i32;
    if mode == "test" {
        file_path = TEST_FILE_PATH;
        (xmin, ymin, xmax, ymax) = (0, 0, 20, 20);
    } else if mode == "real" {
        file_path = FILE_PATH;
        (xmin, ymin, xmax, ymax) = (0, 0, 4000000, 4000000);
    } else {
        panic!("invalid mode");
    }
    let (sensors, _beacons) = get_input(file_path);
    let mut y = ymin;
    while y < ymax {
        let mut x = xmin;
        while x < xmax {
            let max_dist_to_end_of_sensor_area = max_dist_to_end_of_sensor_area((x, y), &sensors);
            if max_dist_to_end_of_sensor_area > 0 {
                x += max_dist_to_end_of_sensor_area;
            } else if max_dist_to_end_of_sensor_area == 0 {
                x += 1;
            } else {
                println!("not in any sensor area at {},{}", x, y);
                let prod: i64 = x as i64 * 4000000 + y as i64;
                return Ok(prod);
            }
        }
        y += 1;
    }
    Err("Did not find the beacon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 4876693);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 11645454855041);
    }
}
