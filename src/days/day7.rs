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
    total_size: i32,
}
impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            files: Vec::new(),
            subdirs: Vec::new(),
            total_size: 0,
        }
    }

    fn find_smallest_dir_size_over_specified_size(&self, required_space: i32) -> i32 {
        let mut smallest_size_that_can_be_deleted: i32 = std::i32::MAX;
        if self.total_size >= required_space && self.total_size < smallest_size_that_can_be_deleted
        {
            smallest_size_that_can_be_deleted = self.total_size;
        }
        for subdir in self.subdirs.iter() {
            let smallest_subdir_size: i32 =
                subdir.find_smallest_dir_size_over_specified_size(required_space);
            if smallest_subdir_size < smallest_size_that_can_be_deleted {
                smallest_size_that_can_be_deleted = smallest_subdir_size;
            }
        }
        return smallest_size_that_can_be_deleted;
    }

    fn get_sum_of_dir_sizes_under_specified_size(&self, size: i32) -> i32 {
        let mut size_sum: i32 = 0;
        if self.total_size <= size {
            size_sum += self.total_size;
        }
        for subdir in self.subdirs.iter() {
            size_sum += subdir.get_sum_of_dir_sizes_under_specified_size(size);
        }
        return size_sum;
    }

    fn add_subdirs_return_where_to_continue_and_size(
        &mut self,
        input: &Vec<Vec<String>>,
        starting_line: usize,
    ) -> (usize, i32) {
        let nr_lines = input.len();
        let mut line_nr: usize = starting_line;
        let mut subdir_size: i32;
        while line_nr < nr_lines - 1 {
            line_nr += 1;
            // println!("{} {}", line_nr, self.name);
            // println!("input line: {:?}", input[line_nr]);
            if input[line_nr][0] == "$" && input[line_nr][1] == "ls" {
            } else if input[line_nr][0] == "dir" {
                let subdir_name = &input[line_nr][1];
                // println!("adding subdir {} to {}", subdir_name, self.name);
                self.subdirs.push(Dir::new(subdir_name.to_string()));
            } else if input[line_nr][0] == "$" && input[line_nr][1] == "cd" {
                let new_dir = &input[line_nr][2];
                match new_dir.as_str() {
                    ".." => return (line_nr, self.total_size),
                    _ => {
                        let index_of_new_dir = self
                            .subdirs
                            .iter()
                            .position(|dir| &dir.name == new_dir)
                            .expect("Could not find subdir.");
                        let subdir = &mut self.subdirs[index_of_new_dir];
                        (line_nr, subdir_size) =
                            subdir.add_subdirs_return_where_to_continue_and_size(input, line_nr);
                        self.total_size += subdir_size;
                    }
                }
            } else {
                // Is a file and size "<size> <file_name>"
                // println!("adding file {} to {}", input[line_nr][1].clone(), self.name);
                let file: File = File {
                    name: input[line_nr][1].clone(),
                    size: input[line_nr][0].parse().expect(
                        format!("Could not make i32 from string. Line nr: {}", line_nr).as_str(),
                    ),
                };
                self.total_size += file.size;
                self.files.push(file);
            }
        }
        return (line_nr, self.total_size);
    }
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    // a line with "dir <dirname>" normally defines the dir in the input. The first line is
    // different though, since it is "/". We define it prior to start reading the input.
    let mut fs = Dir::new("/".to_string());
    (_, _) = fs.add_subdirs_return_where_to_continue_and_size(&input, 0);
    // println!("{:?}", fs);
    let size_sum = fs.get_sum_of_dir_sizes_under_specified_size(100000);

    Ok(size_sum)
}

pub fn result_b() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    // a line with "dir <dirname>" normally defines the dir in the input. The first line is
    // different though, since it is "/". We define it prior to start reading the input.
    let mut fs = Dir::new("/".to_string());
    (_, _) = fs.add_subdirs_return_where_to_continue_and_size(&input, 0);
    let used_space = fs.total_size;
    let free_space = 70000000 - used_space;
    let minimum_required_free_space = 30000000;
    let extra_space_needed = minimum_required_free_space - free_space;
    // println!("{:?}", fs);
    let smallest_size = fs.find_smallest_dir_size_over_specified_size(extra_space_needed);
    Ok(smallest_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 1989474);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 1111607);
    }
}
