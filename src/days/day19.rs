use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day19_test.txt";
//const FILE_PATH: &str = "inputs/day19.txt";

#[derive(Eq, PartialEq, Hash, Debug)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct MatCol {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}
impl MatCol {
    fn new() -> MatCol {
        MatCol {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn from(ore: i32, clay: i32, obsidian: i32, geode: i32) -> MatCol {
        MatCol {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

fn get_input() -> Vec<HashMap<Material, HashMap<Material, i32>>> {
    let mut blueprints: Vec<HashMap<Material, HashMap<Material, i32>>> = Vec::new();
    for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut blueprint: HashMap<Material, HashMap<Material, i32>> = HashMap::new();
        let words: Vec<&str> = line.split_whitespace().collect();
        let ore_robot_cost: i32 = words[6].parse().unwrap();
        let clay_robot_cost: i32 = words[12].parse().unwrap();
        let obsidian_robot_cost_of_ore: i32 = words[18].parse().unwrap();
        let obsidian_robot_cost_of_clay: i32 = words[21].parse().unwrap();
        let geode_robot_cost_of_ore: i32 = words[27].parse().unwrap();
        let geode_robot_cost_of_obsidian: i32 = words[30].parse().unwrap();

        // ore robot
        let costs: HashMap<Material, i32> = HashMap::from([(Material::Ore, ore_robot_cost)]);
        blueprint.insert(Material::Ore, costs);

        // clay robot
        let costs: HashMap<Material, i32> = HashMap::from([(Material::Ore, clay_robot_cost)]);
        blueprint.insert(Material::Clay, costs);

        // obsidian robot
        let costs: HashMap<Material, i32> = HashMap::from([
            (Material::Ore, obsidian_robot_cost_of_ore),
            (Material::Clay, obsidian_robot_cost_of_clay),
        ]);
        blueprint.insert(Material::Obsidian, costs);

        // geode robot
        let costs: HashMap<Material, i32> = HashMap::from([
            (Material::Ore, geode_robot_cost_of_ore),
            (Material::Obsidian, geode_robot_cost_of_obsidian),
        ]);
        blueprint.insert(Material::Geode, costs);

        blueprints.push(blueprint);
    }
    blueprints
}

fn get_blueprint_quality(blueprint: &HashMap<Material, HashMap<Material, i32>>) -> i32 {
    let mut geodes_opened = 0;

    let mut max_single_ore_cost = blueprint[&Material::Ore][&Material::Ore];
    if blueprint[&Material::Clay][&Material::Ore] > max_single_ore_cost {
        max_single_ore_cost = blueprint[&Material::Clay][&Material::Ore];
    }

    // prod of ore, clay, obsidian, geode
    let mut production: MatCol = MatCol::from(1, 0, 0, 0);
    let mut robot_to_build_next: Option<Material> = None;

    // graph containing (minute, shed, robot to build next, production)
    let mut nodes: Vec<(i32, MatCol, Material, MatCol)> = Vec::new();
    nodes.push((1, MatCol::new(), Material::Ore, MatCol::from(1, 0, 0, 0)));
    nodes.push((1, MatCol::new(), Material::Clay, MatCol::from(1, 0, 0, 0)));

    0
}

pub fn result_a() -> Result<i32, &'static str> {
    let blueprints = get_input();
    let mut quality_sum = 0;
    println!("{:?}", blueprints);
    for blueprint in blueprints.iter() {
        quality_sum += get_blueprint_quality(&blueprint);
    }
    Ok(quality_sum)
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
