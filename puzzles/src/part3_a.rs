pub fn compute_power_consumption(diagnostic_strings: Vec<&str>) -> i32 {
    let max_index = diagnostic_strings.get(0).unwrap().len();
    let diagnostics_rows: Vec<Vec<char>> = diagnostic_strings
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for position in 0..max_index {
        let commons = compute_commons(&diagnostics_rows, position);
        gamma.push_str(commons.1);
        epsilon.push_str(commons.0);
    }

    let gamma = isize::from_str_radix(&gamma, 2).unwrap() as i32;
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap() as i32;
    return gamma * epsilon;
}

fn compute_commons(rows: &Vec<Vec<char>>, position: usize) -> (&str, &str) {
    let mut values = Vec::new();
    for row in rows {
        values.push(row.get(position).unwrap());
    }

    let length: f32 = values.len() as f32;
    let sum: u32 = values.iter().map(|num| num.to_digit(10).unwrap()).sum();
    let ratio = (sum as f32) / length;

    return if ratio > 0.5 {
        ("0", "1")
    } else {
        ("1", "0")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_values = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(compute_power_consumption(input_values.to_vec()), 198);
    }
}
