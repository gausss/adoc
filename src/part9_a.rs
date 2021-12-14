use crate::grid::Grid;

pub fn compute_risk_level(input: Vec<&str>) -> i32 {
    let width = input[0].chars().count();
    let grid_input: Vec<i32> = input
        .iter()
        .flat_map(|line| {
            line.chars()
                .into_iter()
                .map(|cell| cell.to_digit(10).unwrap())
                .map(|cell| cell as i32)
        })
        .collect();

    let grid = Grid::new(&grid_input, width);
    let mut lows = vec![];
    for x in 0..grid.width {
        for y in 0..grid.height {
            let cell = grid.cell_at(x as i32, y as i32).unwrap();
            if grid
                .neighbors_at(x as i32, y as i32)
                .iter()
                .all(|neighbor| neighbor.0 > cell)
            {
                lows.push((cell, (x, y)))
            }
        }
    }

    return lows.iter().map(|(height, _)| height + 1).sum()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = include_str!("input/part9_test_input.txt").lines().collect();

        assert_eq!(compute_risk_level(input), 15);
    }

    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part9_input.txt").lines().collect();

        println!("Result 9A: {}", compute_risk_level(input));
    }
}
