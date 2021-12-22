use crate::grid::Grid;

pub fn compute_flashes(input: Vec<&str>) -> i32 {
    let octos: Vec<i32> = input
        .iter()
        .flat_map(|line| {
            line.chars()
                .into_iter()
                .map(|cell| cell.to_digit(10).unwrap())
                .map(|cell| cell as i32)
        })
        .collect();

    let mut octo_grid = Grid::new(&octos, 10);
    let mut count_steps = 0;
    loop {
        count_steps += 1;
        let size = step(&mut octo_grid).len();
        if size == octo_grid.width * octo_grid.height {
            break;
        }
    }

    count_steps
}

fn step(octo_grid: &mut Grid<i32>) -> Vec<(usize, usize)> {
    // 1. increase octo level by 1
    for octo in octo_grid.cells.iter_mut() {
        *octo += 1;
    }

    // 2. flash any octo with level > 9 + flash recursive until no more flashes
    let mut flashed_octos: Vec<(usize, usize)> = vec![];
    loop {
        if !model_flash(octo_grid, &mut flashed_octos) {
            break;
        }
    }

    // 3. set flashed octos level to 0
    for flashed_octo in &flashed_octos {
        octo_grid.set_at(flashed_octo.0, flashed_octo.1, 0);
    }

    flashed_octos
}

fn model_flash(octo_grid: &mut Grid<i32>, flashed_octos: &mut Vec<(usize, usize)>) -> bool {
    let mut flashed = false;
    for x in 0..octo_grid.width {
        for y in 0..octo_grid.height {
            if octo_grid.cell_at(x as i32, y as i32).unwrap() > 9 {
                if !flashed_octos.contains(&(x, y)) {
                    flashed = true;
                    flashed_octos.push((x, y));
                    let neighbors = octo_grid.neighbors_ext_at(x as i32, y as i32);

                    for (octopus, point) in neighbors
                        .iter()
                        .filter(|(_, point)| !flashed_octos.contains(point))
                    {
                        octo_grid.set_at(point.0, point.1, octopus + 1);
                    }
                }
            }
        }
    }

    flashed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = include_str!("input/part11_test_input.txt").lines().collect();

        assert_eq!(compute_flashes(input), 195);
    }

    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part11_input.txt").lines().collect();

        println!("Result 11B: {}", compute_flashes(input));
    }
}
