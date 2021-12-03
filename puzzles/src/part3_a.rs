use math::round;

pub fn compute_power_consumption(diagnostic_strings: Vec<&str>) -> i32 {
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
        .map(|column_value| column_value.to_string())
        .collect::<Vec<String>>();
        
        let gamma_binary = column_binary.join("");
        let epislon_binary = column_binary.iter().map(|f| match f.as_str() {
            "1" => "0",
            "0" => "1",
            _ => "",
        }).map(|f| f.to_string()).collect::<Vec<String>>().join("");

        let gamma = isize::from_str_radix(&gamma_binary, 2).unwrap() as i32;
        let epsilon = isize::from_str_radix(&epislon_binary, 2).unwrap() as i32;
    return gamma * epsilon;
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
