use std::{ops::Add, str::Lines};

pub fn count_digits(input: Lines) -> i32 {
    let mut result: i32 = 0;
    for line in input {
        let split = line.split_once("|").unwrap();
        let one = split
            .0
            .split_whitespace()
            .find(|digit| digit.trim().len() == 2)
            .unwrap();
        let four = split
            .0
            .split_whitespace()
            .find(|digit| digit.trim().len() == 4)
            .unwrap();

        let accumulated = split
            .1
            .split_whitespace()
            .map(|digit| digit.trim())
            .map(|d| match d.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                len => match (
                    len,
                    d.chars()
                        .into_iter()
                        .filter(|&digit| one.contains(digit))
                        .count(),
                    d.chars()
                        .into_iter()
                        .filter(|&digit| four.contains(digit))
                        .count(),
                ) {
                    (5, 1, 3) => 5,
                    (5, 2, 3) => 3,
                    (5, _, 2) => 2,
                    (6, 1, _) => 6,
                    (6, _, 3) => 0,
                    (6, _, 4) => 9,
                    _ => unreachable!(),
                },
            })
            .fold(String::from(""), |acc: String, part: i32| {
                acc.add(&part.to_string())
            });

        let sum: i32 = accumulated.parse().unwrap();
        result += sum;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input8 = include_str!("part8_test_input.txt").lines();

        assert_eq!(count_digits(input8), 61229);
    }
}
