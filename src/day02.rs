pub fn day02(input_lines: &str) -> (String, String) {
    let _ = input_lines;

    for lines in input_lines.lines() {
        let vec: Vec<&str> = lines.split(":").collect();

        let password = vec[1];
        let min: i32 = vec[0].split(" ").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>()[0].parse().expect("Parse Error");
        let max: i32 = vec[0].split(" ").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>()[1].parse().expect("Parse Error");
        let required_char: &str = vec[0].split(" ").collect::<Vec<&str>>()[1];
    }

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(day02("").0, "0".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(day02("").1, "0".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02(""), ("0".to_string(), "0".to_string()))
    }
}
