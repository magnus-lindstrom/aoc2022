pub fn result_a() -> Result<i32, &'static str> {
    let _file_path = "inputs/dayX.txt";
    Ok(0)
}

pub fn result_b() -> Result<i32, &'static str> {
    let _file_path = "inputs/dayX.txt";
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
