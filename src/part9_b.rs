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

    let mut marked = Grid::<bool>::new(&vec![false; grid.width * grid.height], grid.width);
    let mut basins: Vec<usize> = lows
        .into_iter()
        .map(|(_, coords)| basin_at(&grid, coords, &mut marked))
        .map(|b| b.len())
        .collect();

    basins.sort();

    basins[basins.len() - 3..].into_iter().product::<usize>() as i32
}

fn basin_at(map: &Grid<i32>, coords: (usize, usize), marked: &mut Grid<bool>) -> Vec<(i32, (usize, usize))> {
    let (x, y) = (coords.0 as i32, coords.1 as i32);
    let height = map.cell_at(x, y).unwrap();
    let is_marked = marked.cell_at(x, y).unwrap();

    if height >= 9 || is_marked {
        return vec![];
    }

    marked.set_at(coords.0, coords.1, true);

    let neighbors: Vec<(i32, (usize, usize))> = map
        .neighbors_at(x, y)
        .into_iter()
        .filter(|(n, _)| *n > height)
        .map(|(_, point)| basin_at(map, point, marked))
        .flatten()
        .collect();

    vec![vec![(height, coords)], neighbors].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = include_str!("input/part9_test_input.txt").lines().collect();

        assert_eq!(compute_risk_level(input), 1134);
    }

    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part9_input.txt").lines().collect();

        println!("Result 9B: {}", compute_risk_level(input));
    }
}
