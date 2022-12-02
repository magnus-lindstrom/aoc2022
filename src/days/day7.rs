fn result_a() -> Result<i32, &'static str> {

    let _file_path = "inputs/dayX.txt";
    Ok(0)
}

fn result_b() -> Result<i32, &'static str> {

    let _file_path = "inputs/dayX.txt";
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn result_a_is_correct() {
        let answer = result_a();
        assert_eq!(answer, Ok(14375));
    }

    // #[test]
    fn result_b_is_correct() {
        let answer = result_b();
        assert_eq!(answer, Ok(10274));
    }
}
