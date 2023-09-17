use crate::part1_a::count_increasing_measurements;

pub fn count_increasing_sliding_windows(measurements: Vec<i32>) -> i32 {
    let sliding_windows = measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .zip(measurements.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();

    return count_increasing_measurements(sliding_windows);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_values = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increasing_sliding_windows(input_values.to_vec()), 5);
    }

    #[test]
    fn solve() {
        let input: Vec<i32> = include_str!("input/part1_input.txt")
            .split(",")
            .map(|input| input.trim().parse().unwrap())
            .collect();

        println!(
            "Result 1.B: {}",
            count_increasing_sliding_windows(input.to_vec())
        );
    }
}
