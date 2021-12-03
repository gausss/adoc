use math::round;

pub fn compute_life_support(diagnostic_strings: Vec<&str>) -> i32 {
    let width = diagnostic_strings[0].len();
    let height = diagnostic_strings.len();

    let diagnostics = diagnostic_strings
        .into_iter()
        .map(|d_string| d_string.chars().collect::<Vec<char>>())
        .flatten()
        .map(|binary| binary.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut t_diagnostics = vec![0; diagnostics.len()];
    transpose::transpose(&diagnostics, &mut t_diagnostics, width, height);
    let diagnostic_columns = t_diagnostics;

    let column_binary = diagnostic_columns
        .chunks(height)
        .into_iter()
        .map(|diagnostic_column| {
            round::half_up(
                diagnostic_column.iter().sum::<u32>() as f64 / diagnostic_column.len() as f64,
                0,
            )
        })
        .map(|column_value| column_value as u32)
        .collect::<Vec<u32>>();

    let most_common = column_binary
        .iter()
        .map(|result| char::from_digit(*result, 10).unwrap())
        .collect::<Vec<char>>();
    let least_common = column_binary
        .iter()
        .map(|f| match f {
            1 => 0,
            0 => 1,
            _ => 0,
        })
        .map(|f| f as u32)
        .map(|result| char::from_digit(result, 10).unwrap())
        .collect::<Vec<char>>();

    let oxygen_binary = diagnostic_strings
        .into_iter()
        .filter(|val| matches_criteria(val.to_string(), most_common))
        .collect::<Vec<&str>>()[0];

    let co2_binary = diagnostic_strings
        .into_iter()
        .filter(|val| matches_criteria(val.to_string(), least_common))
        .collect::<Vec<&str>>()[0];

    let oxygen = isize::from_str_radix(&oxygen_binary, 2).unwrap() as i32;
    let co2 = isize::from_str_radix(&co2_binary, 2).unwrap() as i32;
    return oxygen * co2;
}

fn matches_criteria(diagnostic_string: String, criteria: Vec<char>) -> bool {
    let chars = diagnostic_string.chars().collect::<Vec<char>>();
    for (position, criteria_at) in criteria.iter().enumerate() {
        if criteria_at.eq(&chars[position]) {
            return false;
        }
    }

    return true;
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
