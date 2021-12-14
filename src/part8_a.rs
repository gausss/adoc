use std::str::Lines;

pub fn count_simple_digits(input: Lines) -> usize {
    return input
        .flat_map(|line| line.split_once("|").unwrap().1.split_whitespace())
        .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input8 = include_str!("input/part8_test_input.txt").lines();

        assert_eq!(count_simple_digits(input8), 26);
    }
    #[test]
    fn solve() {
        let input = include_str!("input/part8_input.txt").lines();
        println!("Result 8.A: {:?}", count_simple_digits(input));
    }
}
