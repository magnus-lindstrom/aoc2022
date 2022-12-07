use crate::utils;

const FILE_PATH: &str = "inputs/day7.txt";

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}
impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            files: Vec::new(),
            subdirs: Vec::new(),
        }
    }
    fn add_subdirs(&mut self, input: &Vec<Vec<String>>, starting_line: usize) -> &mut Dir {
        assert_eq!(input[starting_line][0], "$");
        assert_eq!(input[starting_line][1], "cd");
        let nr_lines = input.len();
        for line_nr in starting_line + 1..nr_lines {
            println!("line number: {}. Current dir: {}", line_nr, self.name);
            println!("{:?}", input[line_nr]);
            if input[line_nr][0] == "$" && input[line_nr][1] == "ls" {
                continue;
            } else if input[line_nr][0] == "dir" {
                let subdir_name = &input[line_nr][1];
                println!("adding subdir {} to {}", subdir_name, self.name);
                self.subdirs.push(Dir::new(subdir_name.to_string()));
            } else if input[line_nr][0] == "$" && input[line_nr][1] == "cd" {
                let new_dir = &input[line_nr][2];
                match new_dir.as_str() {
                    ".." => return self,
                    _ => {
                        // fix this so that it is simply a Dir and not a Vec
                        let index_of_subdir_to_populate = self
                            .subdirs
                            .iter()
                            .position(|dir| &dir.name == new_dir)
                            .expect("Could not find subdir.");
                        let subdir = &mut self.subdirs[index_of_subdir_to_populate];
                        subdir.add_subdirs(input, line_nr);
                    }
                }
            } else {
                // Is a file and size "<size> <file_name>"
                println!("adding file {} to {}", input[line_nr][1].clone(), self.name);
                let file: File = File {
                    name: input[line_nr][1].clone(),
                    size: input[line_nr][0].parse().expect(
                        format!("Could not make i32 from string. Line nr: {}", line_nr).as_str(),
                    ),
                };
                self.files.push(file);
            }
        }
        self
    }
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    // a line with "dir <dirname>" normally defines the dir in the input. The first line is
    // different though, since it is "/". We define it prior to start reading the input.
    let mut fs = Dir::new("/".to_string());
    fs.add_subdirs(&input, 0);
    println!("{:?}", fs);

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
