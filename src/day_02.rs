use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;

const INPUT_DAY_02: &str = "./day_02.txt";

pub fn part1(input_file: &str) -> usize {
    let input = input(input_file);
    let result = input.iter().filter(|row| is_safe(row)).count();
    println!("Day02 part1 = {}", result);
    result
}

pub fn part2(input_file: &str) -> usize {
    let input = input(input_file);
    let result = input.iter().filter(|row| is_safe_dampened(row)).count();
    println!("Day02 part2 = {}", result);
    result
}

pub fn is_safe(row: &Vec<i32>) -> bool {
    let are_all_increasing = are_all_increasing(row);
    let are_all_decreasing = are_all_decreasing(row);
    let have_diff_in_range = have_diff_in_range(row, 1..=3);
    (are_all_increasing || are_all_decreasing) && have_diff_in_range
}

pub fn is_safe_dampened(row: &Vec<i32>) -> bool {
    row.iter().enumerate().any(|(i, _)| {
        let with_level_removed = row.iter().enumerate()
            .filter(|(x, _)| *x != i)
            .map(|(_, v)| *v)
            .collect();
        is_safe(&with_level_removed)
    })
}

fn are_all_increasing(row: &Vec<i32>) -> bool {
    row.windows(2).all(|w| w[0] < w[1])
}

fn are_all_decreasing(row: &Vec<i32>) -> bool {
    row.windows(2).all(|w| w[0] > w[1])
}

fn have_diff_in_range(row: &Vec<i32>, range: RangeInclusive<i32>) -> bool {
    row.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        range.contains(&diff)
    })
}

fn input(input_file: &str) -> Vec<Vec<i32>> {
    let lines = read_lines(input_file);
    lines.iter().map(|line| {
        line.split(" ")
            .map(|s| { s.parse::<i32>().unwrap() })
            .collect()
    }).collect()
}

fn read_lines<P>(file_path: P) -> Vec<String> where P: AsRef<Path>, {
    let name = file_path.as_ref().display().to_string();
    let file = File::open(file_path).expect(format!("Could not open file {}", name).as_str());
    let lines = io::BufReader::new(file).lines();
    lines
        .flatten()
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    const INPUT_DAY_02_TEST: &str = "./day_02_test.txt";

    #[test]
    fn part1() {
        super::part1(super::INPUT_DAY_02);
    }

    #[test]
    fn part2() {
        super::part2(super::INPUT_DAY_02);
    }

    #[test]
    fn part1_test() {
        let result = super::part1(INPUT_DAY_02_TEST);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_test() {
        let result = super::part2(INPUT_DAY_02_TEST);
        assert_eq!(result, 4);
    }

    #[test]
    fn is_safe_test() {
        let input = super::input(INPUT_DAY_02_TEST);
        let expected = [true, false, false, false, false, true];
        input.iter().zip(expected.iter()).for_each(|(r, e)| {
            assert_eq!(super::is_safe(r), *e, "is_safe failed for {:?}, expected {}", r, e);
        });
    }

    #[test]
    fn is_safe_dampened_test() {
        let input = super::input(INPUT_DAY_02_TEST);
        let expected = [true, false, false, true, true, true];
        input.iter().zip(expected.iter()).for_each(|(r, e)| {
            assert_eq!(super::is_safe_dampened(r), *e, "is_safe_dampened failed for {:?}, expected {}", r, e);
        });
    }
}
