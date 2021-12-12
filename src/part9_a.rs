pub fn compute_risk_level(input: Vec<&str>) -> i32 {
    let grid: Vec<Vec<i32>> = input
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|cell| cell.to_digit(10).unwrap())
                .map(|cell| cell as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let empty: Vec<i32> = Vec::new();
    let default: i32 = 9;
    let mut mins: Vec<i32> = Vec::new();
    for (index_y, current_line) in grid.iter().enumerate() {
        for (index_x, current_item) in current_line.iter().enumerate() {
            let mut left = 9;
            if index_x != 0 {
                left = *current_line.get(index_x - 1).unwrap_or(&default);
            }

            let mut right = 9;
            if index_x < current_line.len() {
                right = *current_line.get(index_x + 1).unwrap_or(&default);
            }

            let mut up = 9;
            if index_y != 0 {
                up = *grid
                    .get(index_y - 1)
                    .unwrap_or(&empty)
                    .get(index_x)
                    .unwrap_or(&default);
            }

            let mut down = 9;
            if index_y < grid.len() {
                down = *grid
                    .get(index_y + 1)
                    .unwrap_or(&empty)
                    .get(index_x)
                    .unwrap_or(&default);
            }

            let adjacent = vec![left, right, up, down];
            let adjacent_min = adjacent.iter().min().unwrap();

            if &adjacent_min > &current_item {
                println!("Found min {}", &current_item);
                mins.push(*current_item + 1);
            }
        }
    }

    return mins.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = include_str!("part9_input.txt").lines().collect();

        assert_eq!(compute_risk_level(input), 15);
    }
}
