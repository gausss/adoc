pub fn count_increasing_sliding_windows(measurements: Vec<i32>) -> i32 {
    let mut sliding_windows = Vec::new();

    for (position, _current_value) in measurements.iter().enumerate() {
        if position == 0 || position >= measurements.len() - 1 {
            continue;
        }

        let sliding_window_sum = measurements[position-1..position+2].iter().sum();
        sliding_windows.push(sliding_window_sum);
    }

    return count_increasing_measurements(sliding_windows);
}

fn count_increasing_measurements(measurements: Vec<i32>) -> i32 {
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

        assert_eq!(count_increasing_sliding_windows(input_values.to_vec()), 5);
    }
}
