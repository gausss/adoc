pub fn count_increasing_measurements(measurements: Vec<i32>) -> i32 {
    let mut count = 0;

    for (position, current_value) in measurements.iter().enumerate() {
        if position == 0 {
            continue;
        }

        let previous_value = measurements[position - 1];
        if current_value > &previous_value {
            count = count + 1;
        }
    }

    return count;
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
