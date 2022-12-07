use crate::utils;

const FILE_PATH: &str = "inputs/day7.txt";

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    println!("{:?}", input);

    // (dir name, dir size, subdirs)
    let mut file_system: Vec<(String, i32, Vec<String>)> = Vec::new();
    let mut upper_dirs: String = "".to_string();
    for line in input.iter() {
        if line[0] == "cd" {
            // let dir: (String, i32, Vec<String>) = (upper_dirs + &line[1], 0, Vec::new());
            let dir_name: String;
            let dir_size = 0;
            let subdirs: Vec<String> = Vec::new();

            match line[1].as_str() {
                // remove last dir in path: /a/b/c -> /a/b
                ".." => upper_dirs = upper_dirs.rsplit_once('/').unwrap().0.to_string(),
                dir => {
                    dir_name = format!("{}/{}", upper_dirs, dir);
                    upper_dirs = dir_name.clone();
                    file_system.push((dir_name, dir_size, subdirs));
                }
            }
        }
    }

    Ok(0)
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
