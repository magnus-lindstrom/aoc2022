use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day18.txt";
//const FILE_PATH: &str = "inputs/day18_test.txt";

#[derive(PartialEq, Debug)]
enum Air {
    Inside,
    Outside,
}

struct Bounds {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    minz: i32,
    maxz: i32,
}
impl Bounds {
    fn new() -> Bounds {
        Bounds {
            minx: std::i32::MAX,
            maxx: std::i32::MIN,
            miny: std::i32::MAX,
            maxy: std::i32::MIN,
            minz: std::i32::MAX,
            maxz: std::i32::MIN,
        }
    }
}

fn get_input_with_bounds() -> (Vec<Vec<i32>>, HashMap<Vec<i32>, Air>, Bounds) {
    let file_contents: String = std::fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());
    let mut bounds: Bounds = Bounds::new();
    let mut lava: Vec<Vec<i32>> = Vec::new();
    let mut air: HashMap<Vec<i32>, Air> = HashMap::new();
    for line in file_contents.lines() {
        let v: Vec<i32> = line.split(',').map(|e| e.parse().unwrap()).collect();
        if v[0] < bounds.minx {
            bounds.minx = v[0];
        } else if v[0] > bounds.maxx {
            bounds.maxx = v[0];
        }
        if v[1] < bounds.miny {
            bounds.miny = v[1];
        } else if v[1] > bounds.maxy {
            bounds.maxy = v[1];
        }
        if v[2] < bounds.minz {
            bounds.minz = v[2];
        } else if v[2] > bounds.maxz {
            bounds.maxz = v[2];
        }
        lava.push(v);
    }
    for x in bounds.minx..=bounds.maxx {
        for y in bounds.miny..=bounds.maxy {
            for z in bounds.minz..=bounds.maxz {
                let point = vec![x, y, z];
                if !lava.contains(&point) {
                    air.insert(point, Air::Inside);
                }
            }
        }
    }

    (lava, air, bounds)
}

fn get_input_without_bounds() -> Vec<Vec<i32>> {
    let file_contents: String = std::fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());
    let mut output: Vec<Vec<i32>> = Vec::new();
    for line in file_contents.lines() {
        let v: Vec<i32> = line.split(',').map(|e| e.parse().unwrap()).collect();
        output.push(v);
    }
    output
}

fn is_adjacent(p1: &Vec<i32>, p2: &Vec<i32>) -> bool {
    let mut found_diff_of_one = false;
    for (p1_elem, p2_elem) in p1.iter().zip(p2.iter()) {
        let diff = p1_elem.abs_diff(*p2_elem);
        if diff == 1 {
            if found_diff_of_one {
                return false;
            } else {
                found_diff_of_one = true;
            }
        } else if diff > 1 {
            return false;
        }
    }
    true
}

fn get_sides_touching_inside_air(point: Vec<i32>, air: &HashMap<Vec<i32>, Air>) -> i32 {
    let mut sides_touching_inside_air = 0;
    let (mut p1, mut p2, mut p3, mut p4, mut p5, mut p6) = (
        point.clone(),
        point.clone(),
        point.clone(),
        point.clone(),
        point.clone(),
        point.clone(),
    );
    p1[0] -= 1;
    p2[0] += 1;
    p3[1] -= 1;
    p4[1] += 1;
    p5[2] -= 1;
    p6[2] += 1;
    if air.get(&p1) == Some(&Air::Inside) {
        sides_touching_inside_air += 1;
    }
    if air.get(&p2) == Some(&Air::Inside) {
        sides_touching_inside_air += 1;
    }
    if air.get(&p3) == Some(&Air::Inside) {
        sides_touching_inside_air += 1;
    }
    if air.get(&p4) == Some(&Air::Inside) {
        sides_touching_inside_air += 1;
    }
    if air.get(&p5) == Some(&Air::Inside) {
        sides_touching_inside_air += 1;
    }
    if air.get(&p6) == Some(&Air::Inside) {
        sides_touching_inside_air += 1;
    }
    sides_touching_inside_air
}

fn count_sides_considering_air(
    points: &mut Vec<Vec<i32>>,
    index: usize,
    air: &HashMap<Vec<i32>, Air>,
) -> i32 {
    let mut sides = 0;
    if points.is_empty() {
        return sides;
    }
    let point = points.remove(index);
    sides += 6;
    let mut adjacent_cubes = 0;
    let mut adjacent_indeces: Vec<usize> = Vec::new();
    for i in 0..points.len() {
        if is_adjacent(&point, &points[i]) {
            adjacent_cubes += 1;
            adjacent_indeces.push(i);
        }
    }
    for i in adjacent_indeces.into_iter() {
        sides += count_sides_considering_air(points, i, air) - 1; // -1 because they all touch this point
    }
    sides -= adjacent_cubes;

    sides -= get_sides_touching_inside_air(point, &air);

    if points.len() > 0 {
        sides += count_sides_considering_air(points, 0, air);
    }
    sides
}

fn count_sides(points: &mut Vec<Vec<i32>>, index: usize) -> i32 {
    let mut sides = 0;
    if points.is_empty() {
        return sides;
    }
    let point = points.remove(index);
    sides += 6;
    let mut adjacent_cubes = 0;
    let mut adjacent_indeces: Vec<usize> = Vec::new();
    for i in 0..points.len() {
        if is_adjacent(&point, &points[i]) {
            adjacent_cubes += 1;
            adjacent_indeces.push(i);
        }
    }
    for i in adjacent_indeces.iter() {
        sides += count_sides(points, *i) - 1; // -1 because they all touch this point
    }
    sides -= adjacent_cubes;

    if points.len() > 0 {
        sides += count_sides(points, 0);
    }
    sides
}

fn adjacent_to_outside_air(point: (i32, i32, i32), air: &mut HashMap<Vec<i32>, Air>) -> bool {
    if air.get(&vec![point.0 + 1, point.1, point.2]) == Some(&Air::Outside)
        || air.get(&vec![point.0 - 1, point.1, point.2]) == Some(&Air::Outside)
        || air.get(&vec![point.0, point.1 + 1, point.2]) == Some(&Air::Outside)
        || air.get(&vec![point.0, point.1 - 1, point.2]) == Some(&Air::Outside)
        || air.get(&vec![point.0, point.1, point.2 + 1]) == Some(&Air::Outside)
        || air.get(&vec![point.0, point.1, point.2 - 1]) == Some(&Air::Outside)
    {
        // if point == (2, 3, 4) {
        // println!("point is (2,3,4). Is adjacent to outside air");
        // }

        return true;
    }
    // if point == (2, 3, 4) {
    // println!("point is (2,3,4). Is not adjacent to outside air");
    // }
    false
}

fn mark_outside_air(air: &mut HashMap<Vec<i32>, Air>, bounds: &Bounds) -> () {
    loop {
        let changed = mark_outside_air_return_if_changed(air, &bounds);
        if !changed {
            break;
        }
    }
}

fn mark_outside_air_return_if_changed(air: &mut HashMap<Vec<i32>, Air>, b: &Bounds) -> bool {
    let mut changed = false;

    // set all sides to outside air
    let x_ranges: Vec<i32> = vec![b.minx, b.maxx];
    for x in x_ranges.into_iter() {
        for y in b.miny..=b.maxy {
            for z in b.minz..=b.maxz {
                let key = vec![x, y, z];
                if air.contains_key(&key) {
                    *air.get_mut(&key).unwrap() = Air::Outside;
                }
            }
        }
    }
    let y_ranges: Vec<i32> = vec![b.miny, b.maxy];
    for y in y_ranges.into_iter() {
        for x in b.minx..=b.maxx {
            for z in b.minz..=b.maxz {
                let key = vec![x, y, z];
                if air.contains_key(&key) {
                    *air.get_mut(&key).unwrap() = Air::Outside;
                }
            }
        }
    }
    let z_ranges: Vec<i32> = vec![b.minz, b.maxz];
    for z in z_ranges.into_iter() {
        for x in b.minx..=b.maxx {
            for y in b.miny..=b.maxy {
                let key = vec![x, y, z];
                if air.contains_key(&key) {
                    *air.get_mut(&key).unwrap() = Air::Outside;
                }
            }
        }
    }

    // set inside squares to inside air, if not connected to outside air
    for x in b.minx..=b.maxx {
        for y in b.miny..=b.maxy {
            for z in b.minz..=b.maxz {
                let key = vec![x, y, z];
                // println!("looking at key {:?}", key);
                if air.contains_key(&key)
                    && adjacent_to_outside_air((x, y, z), air)
                    && air[&key] == Air::Inside
                {
                    *air.get_mut(&key).unwrap() = Air::Outside;
                    changed = true;
                }
            }
        }
    }
    changed
}

pub fn result_a() -> Result<i32, &'static str> {
    let mut points = get_input_without_bounds();
    let sides: i32 = count_sides(&mut points, 0);
    Ok(sides)
}

pub fn result_b() -> Result<i32, &'static str> {
    let (mut lava, mut air, bounds) = get_input_with_bounds();
    mark_outside_air(&mut air, &bounds);
    let sides: i32 = count_sides_considering_air(&mut lava, 0, &air);

    Ok(sides)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 4370);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 2458);
    }
}
