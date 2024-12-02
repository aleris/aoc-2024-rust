use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT_DAY_01: &str = "./day_01.txt";

pub fn part1(input_file: &str) -> i32 {
    let (left, right) = input(input_file);
    let mut left = left;
    left.sort();
    let mut right = right;
    right.sort();
    let list: Vec<(i32, i32)> = left.into_iter().zip(right.into_iter()).collect();
    let result = list.iter().map(|(l, r)| (l - r).abs()).sum::<i32>();
    println!("Day01 part1 = {}", result);
    result
}

pub fn part2(input_file: &str) -> i32 {
    let (left, right) = input(input_file);
    let right_map_counts = right.iter().fold(std::collections::HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let result = left.into_iter().map(|x| {
        let right_count = right_map_counts.get(&x).unwrap_or(&0);
        x * right_count
    }).sum::<i32>();
    println!("Day01 part2 = {}", result);
    result
}

fn input(input_file: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = read_lines(input_file);
    let list_of_pairs: Vec<(i32, i32)> = lines.iter().map(|line| {
        let row: Vec<i32> = line.split("   ")
            .map(|s| { s.parse::<i32>().unwrap() })
            .collect();
        (row[0], row[1])
    }).collect();
    list_of_pairs.into_iter().unzip()
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
    const INPUT_DAY_01_TEST: &str = "./day_01_test.txt";

    #[test]
    fn part1() {
        super::part1(super::INPUT_DAY_01);
    }

    #[test]
    fn part2() {
        super::part2(super::INPUT_DAY_01);
    }

    #[test]
    fn part1_test() {
        let result = super::part1(INPUT_DAY_01_TEST);
        assert_eq!(result, 11);
    }

    #[test]
    fn part2_test() {
        let result = super::part2(INPUT_DAY_01_TEST);
        assert_eq!(result, 31);
    }
}
