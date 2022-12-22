use queue::Queue;
use std::collections::HashSet;
use std::fs;

const FILE_PATH: &str = "inputs/day6.txt";

fn get_pos_of_past_uniqueness(string: &str, unique_len: i32) -> Result<i32, String> {
    let mut past_chars: Queue<char> = Queue::with_capacity(unique_len as usize);

    let mut msg_pos: i32 = 1;
    for ch in string.chars() {
        past_chars.force_queue(ch);
        let hset: HashSet<&char> = past_chars.vec().into_iter().collect::<HashSet<_>>();
        if hset.len() == unique_len as usize && msg_pos >= unique_len {
            return Ok(msg_pos);
        }
        msg_pos += 1;
    }

    Err(format!(
        "Could not find position in message with {} preceding unique chars.",
        unique_len
    ))
}

pub fn result_a() -> Result<i32, String> {
    /* Input is a long string of characters.
     * First the point where the preceding 4 characters are all unique.
     */
    let file_contents: &str = &fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());
    let message = file_contents.trim();

    get_pos_of_past_uniqueness(message, 4)
}

pub fn result_b() -> Result<i32, String> {
    /* Input is a long string of characters.
     * First the point where the preceding 14 characters are all unique.
     */
    let file_contents: &str = &fs::read_to_string(FILE_PATH)
        .expect(format!("Could not read file '{}'", FILE_PATH).as_str());
    let message = file_contents.trim();

    get_pos_of_past_uniqueness(message, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 1640);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 3613);
    }
}
