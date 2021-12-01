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
    fn test_add() {
        let input_values: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increasing_measurements(input_values.to_vec()), 7);
    }
}
