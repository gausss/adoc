pub fn count_increasing_measurements(measurements: Vec<i32>) -> i32 {
    return measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_values = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increasing_measurements(input_values.to_vec()), 7);

        println!("{}", input_values.len());
    }

    #[test]
    fn solve() {
        let input: Vec<i32> = include_str!("input/part1_input.txt")
            .split(",")
            .map(|input| input.trim().parse().unwrap())
            .collect();

        println!(
            "Result 1.A: {}",
            count_increasing_measurements(input.to_vec())
        );
    }
}
