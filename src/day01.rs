use itertools::Itertools;

pub fn day01(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let mut answer1: i32 = 0;
    let mut answer2 = 0;

    for combination in input_lines.lines().combinations(2) {
        let numbers: Vec<i32> = combination.into_iter().map(|s| s.parse().expect("parse error")).collect();
        if numbers.iter().sum::<i32>() == 2020 {
            answer1 = numbers.iter().product::<i32>()
        }

    }
    for combination in input_lines.lines().combinations(3) {
        let numbers: Vec<i32> = combination.into_iter().map(|s| s.parse().expect("parse error")).collect();
        if numbers.iter().sum::<i32>() == 2020 {
            answer2 = numbers.iter().product::<i32>()
        }
    }

    (format!("{}", answer1), format!("{}", answer2))
}