use std::fs;

const INPUT_DAY_03: &str = "./day_03.txt";

const DISABLE_TOKEN: &str = "don't()";
const ENABLE_TOKEN: &str = "do()";
const MUL_START_TOKEN: &str = "mul(";
const MUL_SEPARATOR_TOKEN: &str = ",";
const MUL_END_TOKEN: &str = ")";

pub fn part1(input_file: &str) -> i32 {
    let input = input(input_file);
    let result = add_mul(find_all_mul(input, false));
    println!("Day03 part1 = {}", result);
    result
}

pub fn part2(input_file: &str) -> i32 {
    let input = input(input_file);
    let result = add_mul(find_all_mul(input, true));
    println!("Day03 part2 = {}", result);
    result
}

fn find_all_mul(input: String, allow_disable: bool) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut enabled = true;
    input.char_indices().for_each(|(i, _c)| {
        if i >= ENABLE_TOKEN.len() && &input[i - ENABLE_TOKEN.len()..i] == ENABLE_TOKEN {
            enabled = true;
        }
        if enabled {
            if allow_disable
                && i >= DISABLE_TOKEN.len()
                && (&input[i - DISABLE_TOKEN.len()..i] == DISABLE_TOKEN) {
                enabled = false;
            }
            if i >= MUL_START_TOKEN.len() && &input[i - MUL_START_TOKEN.len()..i] == MUL_START_TOKEN {
                match &input[i..].find(MUL_END_TOKEN) {
                    None => (),
                    Some(end) => {
                        match try_extract_digits(&input[i..i + end]) {
                            Some((d1, d2)) => result.push((d1, d2)),
                            None => (),
                        }
                    }
                }
            }
        }
    });
    result
}

fn try_extract_digits(input: &str) -> Option<(i32, i32)> {
    let digits: Vec<&str> = input
        .split(MUL_SEPARATOR_TOKEN)
        .collect();
    if digits.len() == 2 {
        let d1 = digits[0].parse::<i32>();
        let d2 = digits[1].parse::<i32>();
        if d1.is_ok() && d2.is_ok() {
            Some((d1.unwrap(), d2.unwrap()))
        } else {
            None
        }
    } else {
        None
    }
}

fn add_mul(list: Vec<(i32, i32)>) -> i32 {
    list.iter().map(|(a, b)| a * b).sum::<i32>()
}

fn input(input_file: &str) -> String {
    fs::read_to_string(input_file).expect(format!("Could not open file {}", input_file).as_str())
}

#[cfg(test)]
mod tests {
    const INPUT_DAY_03_TEST: &str = "./day_03_test.txt";
    const INPUT_DAY_03_TEST_B: &str = "./day_03_test_b.txt";

    #[test]
    fn part1() {
        super::part1(super::INPUT_DAY_03);
    }

    #[test]
    fn part2() {
        super::part2(super::INPUT_DAY_03);
    }

    #[test]
    fn part1_test() {
        let result = super::part1(INPUT_DAY_03_TEST);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_test() {
        let result = super::part2(INPUT_DAY_03_TEST_B);
        assert_eq!(result, 48);
    }

    #[test]
    fn find_all_mul() {
        let input = super::input(INPUT_DAY_03_TEST);
        let result = super::find_all_mul(input, false);
        assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }
}
