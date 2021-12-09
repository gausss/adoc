pub fn compute_life_support(diagnostic_strings: Vec<&str>) -> i32 {
    let max_index = diagnostic_strings.get(0).unwrap().len();
    let diagnostics_rows: Vec<Vec<char>> = diagnostic_strings
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut oxygen_candidates = diagnostics_rows.clone();
    let mut carbo_candidates = diagnostics_rows.clone();
    for position in 0..max_index {
        if oxygen_candidates.len() > 1 {
            let commons_oxygen = compute_commons(&oxygen_candidates, position);
            oxygen_candidates = clean_candidates(&mut oxygen_candidates, position, &commons_oxygen.1);
        }

        if carbo_candidates.len() > 1 {
            let commons_carbo = compute_commons(&carbo_candidates, position);
            carbo_candidates = clean_candidates(&mut carbo_candidates, position, &commons_carbo.0);
        }
    }

    let oxygen_string: String = oxygen_candidates.get(0).unwrap().iter().map(|digit| digit.to_string()).collect();
    let carbo_string: String = carbo_candidates.get(0).unwrap().iter().map(|digit| digit.to_string()).collect();

    let oxygen = isize::from_str_radix(&oxygen_string, 2).unwrap() as i32;
    let carbo = isize::from_str_radix(&carbo_string, 2).unwrap() as i32;
    return oxygen * carbo;
}

fn compute_commons(rows: &Vec<Vec<char>>, position: usize) -> (char, char) {
    let mut values = Vec::new();
    for row in rows {
        values.push(row.get(position).unwrap());
    }

    let length: f32 = values.len() as f32;
    let sum: u32 = values.iter().map(|num| num.to_digit(10).unwrap()).sum();
    let ratio = (sum as f32) / length;

    return if ratio >= 0.5 {
        ('0', '1')
    } else {
        ('1', '0')
    };
}

fn clean_candidates(rows: &mut Vec<Vec<char>>, position: usize, match_value: &char) -> Vec<Vec<char>> {
    let mut values = Vec::new();
    for row in rows {
        if row.get(position).unwrap() == match_value {
            values.push(row.clone());
        }
    }

    return values;
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

        assert_eq!(compute_life_support(input_values.to_vec()), 230);
    }
}
