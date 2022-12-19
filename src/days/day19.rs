use std::collections::HashMap;
//const FILE_PATH: &str = "inputs/day19_test.txt";
const FILE_PATH: &str = "inputs/day19.txt";

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
struct Storage {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}
impl Storage {
    fn new() -> Storage {
        Storage {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn add(&mut self, production: &HashMap<Material, i32>) -> () {
        self.ore += production[&Material::Ore];
        self.clay += production[&Material::Clay];
        self.obsidian += production[&Material::Obsidian];
        self.geode += production[&Material::Geode];
    }
    fn has_mats_for_robot(&self, blueprint: &HashMap<Material, i32>) -> bool {
        if self.ore >= blueprint[&Material::Ore]
            && self.clay >= blueprint[&Material::Clay]
            && self.obsidian >= blueprint[&Material::Obsidian]
            && self.geode >= blueprint[&Material::Geode]
        {
            return true;
        }
        false
    }
    fn build_robot(&mut self, blueprint: &HashMap<Material, i32>) -> () {
        self.ore -= blueprint[&Material::Ore];
        self.clay -= blueprint[&Material::Clay];
        self.obsidian -= blueprint[&Material::Obsidian];
        self.geode -= blueprint[&Material::Geode];
    }
}

fn get_input() -> Vec<HashMap<Material, HashMap<Material, i32>>> {
    let mut vec_of_blueprints: Vec<HashMap<Material, HashMap<Material, i32>>> = Vec::new();
    for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut blueprints: HashMap<Material, HashMap<Material, i32>> = HashMap::new();
        let words: Vec<&str> = line.split_whitespace().collect();
        let ore_robot_cost: i32 = words[6].parse().unwrap();
        let clay_robot_cost: i32 = words[12].parse().unwrap();
        let obsidian_robot_cost_of_ore: i32 = words[18].parse().unwrap();
        let obsidian_robot_cost_of_clay: i32 = words[21].parse().unwrap();
        let geode_robot_cost_of_ore: i32 = words[27].parse().unwrap();
        let geode_robot_cost_of_obsidian: i32 = words[30].parse().unwrap();

        // ore robot costs
        let costs: HashMap<Material, i32> = HashMap::from([
            (Material::Ore, ore_robot_cost),
            (Material::Clay, 0),
            (Material::Obsidian, 0),
            (Material::Geode, 0),
        ]);
        blueprints.insert(Material::Ore, costs);

        // clay robot costs
        let costs: HashMap<Material, i32> = HashMap::from([
            (Material::Ore, clay_robot_cost),
            (Material::Clay, 0),
            (Material::Obsidian, 0),
            (Material::Geode, 0),
        ]);
        blueprints.insert(Material::Clay, costs);

        // obsidian robot costs
        let costs: HashMap<Material, i32> = HashMap::from([
            (Material::Ore, obsidian_robot_cost_of_ore),
            (Material::Clay, obsidian_robot_cost_of_clay),
            (Material::Obsidian, 0),
            (Material::Geode, 0),
        ]);
        blueprints.insert(Material::Obsidian, costs);

        // geode robot costs
        let costs: HashMap<Material, i32> = HashMap::from([
            (Material::Ore, geode_robot_cost_of_ore),
            (Material::Clay, 0),
            (Material::Obsidian, geode_robot_cost_of_obsidian),
            (Material::Geode, 0),
        ]);
        blueprints.insert(Material::Geode, costs);

        vec_of_blueprints.push(blueprints);
    }
    vec_of_blueprints
}

fn sum_of_n_to_n_plus_x(n: i32, x: i32) -> i32 {
    (n..n + x).fold(0, |a, b| a + b)
}

fn impossible_to_beat_max(
    max_geodes_opened: i32,
    time_left: i32,
    storage: &Storage,
    production: &HashMap<Material, i32>,
) -> bool {
    if max_geodes_opened
        >= storage.geode + time_left * sum_of_n_to_n_plus_x(production[&Material::Geode], time_left)
    {
        return true;
    }
    false
}

fn get_max_production_needed(
    blueprints: &HashMap<Material, HashMap<Material, i32>>,
) -> (i32, i32, i32) {
    let mut max_ore = 0;
    let mut max_clay = 0;
    let mut max_obsidian = 0;
    for robot in blueprints.keys() {
        if blueprints[robot][&Material::Ore] > max_ore {
            max_ore = blueprints[robot][&Material::Ore];
        }
        if blueprints[robot][&Material::Clay] > max_clay {
            max_clay = blueprints[robot][&Material::Clay];
        }
        if blueprints[robot][&Material::Obsidian] > max_obsidian {
            max_obsidian = blueprints[robot][&Material::Obsidian];
        }
    }
    (max_ore, max_clay, max_obsidian)
}

fn get_blueprint_max_geodes(
    blueprints: &HashMap<Material, HashMap<Material, i32>>,
    minutes: i32,
) -> i32 {
    let mut max_geodes_opened = 0;
    let (max_ore_needed, max_clay_needed, max_obsidian_needed) =
        get_max_production_needed(&blueprints);

    let init_production: HashMap<Material, i32> = HashMap::from([
        (Material::Ore, 1),
        (Material::Clay, 0),
        (Material::Obsidian, 0),
        (Material::Geode, 0),
    ]);

    // graph containing (minute, storage, robot to build next, production)
    let mut nodes: Vec<(i32, Storage, Material, HashMap<Material, i32>)> = Vec::new();
    // two possible starting plans, building either ore or clay robots
    nodes.push((
        minutes,
        Storage::new(),
        Material::Ore,
        init_production.clone(),
    ));
    nodes.push((minutes, Storage::new(), Material::Clay, init_production).clone());

    while nodes.len() > 0 {
        let (mut time_left, mut storage, robot_to_build, mut production) = nodes.pop().unwrap();
        let mut robot_was_built = false;
        while !robot_was_built {
            if storage.has_mats_for_robot(&blueprints[&robot_to_build]) {
                storage.build_robot(&blueprints[&robot_to_build]);

                robot_was_built = true;
            }

            storage.add(&production);

            if robot_was_built {
                *production.get_mut(&robot_to_build).unwrap() += 1;
            }

            time_left -= 1;

            // don't add any more nodes to search from this point if it is impossible to beat the
            // high score
            if impossible_to_beat_max(max_geodes_opened, time_left, &storage, &production) {
                break;
            }

            if time_left == 0 {
                if storage.geode > max_geodes_opened {
                    max_geodes_opened = storage.geode;
                }
                break;
            } else if robot_was_built {
                if production[&Material::Ore] < max_ore_needed {
                    nodes.push((
                        time_left,
                        storage.clone(),
                        Material::Ore,
                        production.clone(),
                    ));
                }
                if production[&Material::Clay] < max_clay_needed {
                    nodes.push((
                        time_left,
                        storage.clone(),
                        Material::Clay,
                        production.clone(),
                    ));
                }

                if production[&Material::Clay] > 0
                    && production[&Material::Obsidian] < max_obsidian_needed
                {
                    nodes.push((
                        time_left,
                        storage.clone(),
                        Material::Obsidian,
                        production.clone(),
                    ));
                }
                if production[&Material::Obsidian] > 0 {
                    nodes.push((
                        time_left,
                        storage.clone(),
                        Material::Geode,
                        production.clone(),
                    ));
                }
            }
        }
    }

    max_geodes_opened
}

/// depth first search, prune results that can not theoretically perform as highly
///
pub fn result_a() -> Result<i32, &'static str> {
    println!("{}", sum_of_n_to_n_plus_x(5, 1));
    let vec_of_blueprints = get_input();
    let minutes = 24;
    let mut quality_sum = 0;
    for (i, blueprints) in vec_of_blueprints.iter().enumerate() {
        quality_sum += (i as i32 + 1) * get_blueprint_max_geodes(&blueprints, minutes);
        println!("done with blueprints {}", i + 1);
    }
    Ok(quality_sum)
}

pub fn result_b() -> Result<i32, &'static str> {
    let vec_of_blueprints = get_input();
    let minutes = 32;
    let mut geode_product = 1;
    for (i, blueprints) in vec_of_blueprints[0..3].iter().enumerate() {
        geode_product *= get_blueprint_max_geodes(&blueprints, minutes);
        println!("done with blueprints {}", i + 1);
    }
    Ok(geode_product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 1144);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 19980);
    }
}
