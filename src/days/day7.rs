use crate::utils;

const FILE_PATH: &str = "inputs/day7.txt";

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Dir>,
    total_size: i32,
}
impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            subdirs: Vec::new(),
            total_size: 0,
        }
    }

    /// Recursive function. Returns smallest directory size that satisfies size > required_space.
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

    /// Recursive function. Returns sum of dir sizes below specified size.
    ///
    /// For all directories with size <= max_size, returns the sum of their sizes.
    fn get_sum_of_dir_sizes_under_specified_size(&self, max_size: i32) -> i32 {
        let mut size_sum: i32 = 0;
        if self.total_size <= max_size {
            size_sum += self.total_size;
        }
        for subdir in self.subdirs.iter() {
            size_sum += subdir.get_sum_of_dir_sizes_under_specified_size(max_size);
        }
        return size_sum;
    }

    /// Recursive function. Returns total space and last parsed line.
    ///
    /// Function will add subdirectories to self from the input lines in <input>.
    /// It returns the line where either a "cd .." was encountered, or where the input ended.
    /// It also returns the total size of all directories and files beneath it.
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
            if input[line_nr][0] == "$" && input[line_nr][1] == "ls" {
                // Do nothing
            } else if input[line_nr][0] == "dir" {
                let subdir_name = &input[line_nr][1];
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
                // Line indicates a file and its size: "<size> <file_name>"
                let file_size: i32 = input[line_nr][0].parse().expect(
                    format!("Could not make i32 from string. Line nr: {}", line_nr).as_str(),
                );
                self.total_size += file_size;
            }
        }
        return (line_nr, self.total_size);
    }
}

pub fn result_a() -> Result<i32, &'static str> {
    let max_size_of_dirs: i32 = 100000;
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    let mut fs = Dir::new("/".to_string());
    // Populate file system, output of recursive function is not used at this level. Discard.
    (_, _) = fs.add_subdirs_return_where_to_continue_and_size(&input, 0);
    let size_sum = fs.get_sum_of_dir_sizes_under_specified_size(max_size_of_dirs);

    Ok(size_sum)
}

/// Finds the smallest directory that can be deleted to free up enough space.
pub fn result_b() -> Result<i32, &'static str> {
    let total_space_on_machine: i32 = 70000000;
    let minimum_required_free_space: i32 = 30000000;
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);

    let mut fs = Dir::new("/".to_string());

    // Populate file system, output of recursive function is not used at this level. Discard.
    (_, _) = fs.add_subdirs_return_where_to_continue_and_size(&input, 0);

    let used_space = fs.total_size;
    let free_space = total_space_on_machine - used_space;
    let extra_space_needed = minimum_required_free_space - free_space;

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
