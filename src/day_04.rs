use std::fs;

const INPUT_DAY_04: &str = "./day_04.txt";
const TARGET: &[char; 4] = &['X', 'M', 'A', 'S'];
const ALL_DIRECTIONS: &[(i32, i32); 8] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

pub fn part1(input_file: &str) -> i32 {
    let input = input(input_file);
    let result = search(&input);
    println!("Day04 part1 = {}", result);
    result
}

pub fn part2(input_file: &str) -> i32 {
    let input = input(input_file);
    let result = search_flip(&input);
    println!("Day04 part2 = {}", result);
    result
}

fn search(input: &Vec<Vec<char>>) -> i32 {
    let mut found = 0;
    input.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            ALL_DIRECTIONS.iter().for_each(|(dx, dy)| {
                if search_direction(input, x, y, *dx, *dy) {
                    found += 1;
                }
            })
        });
    });
    found
}

fn search_direction(input: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    TARGET.iter().enumerate().all(|(i, target)| {
        let nx = x as i32 + dx * i as i32;
        let ny = y as i32 + dy * i as i32;
        if nx < 0 || ny < 0 || nx >= input[0].len() as i32 || ny >= input.len() as i32 {
            false
        } else {
            let nx = nx as usize;
            let ny = ny as usize;
            if input[ny][nx] != *target {
                false
            } else {
                true
            }
        }
    })
}

fn search_flip(input: &Vec<Vec<char>>) -> i32 {
    let mut found = 0;
    input.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            if is_flip(input, x, y) {
                found += 1;
            }
        });
    });
    found
}

fn is_flip(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if input[y][x] != 'A' {
        return false;
    }
    if x == 0 || y == 0 || x == input[0].len() - 1 || y == input.len() - 1 {
        return false;
    }
    let x1 = (input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S') ||
        (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M');

    let x2 = (input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S') ||
        (input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M');

    x1 && x2
}

fn input(input_file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input_file).expect(format!("Could not open file {}", input_file)
        .as_str())
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT_DAY_04_TEST: &str = "./day_04_test.txt";
    const INPUT_DAY_04_TEST_SIMPLE: &str = "./day_04_test_simple.txt";

    #[test]
    fn part1() {
        super::part1(super::INPUT_DAY_04);
    }

    #[test]
    fn part2() {
        super::part2(super::INPUT_DAY_04);
    }

    #[test]
    fn part1_test() {
        let result = super::part1(INPUT_DAY_04_TEST);
        assert_eq!(result, 18);
    }

    #[test]
    fn part1_simple_test() {
        let result = super::part1(INPUT_DAY_04_TEST_SIMPLE);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_test() {
        let result = super::part2(INPUT_DAY_04_TEST);
        assert_eq!(result, 9);
    }
}
