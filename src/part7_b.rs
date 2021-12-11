pub fn cheapest_increasing_alignment(crabs: &Vec<i32>) -> i32 {
    let min = crabs.iter().min().unwrap().clone();
    let max = crabs.iter().max().unwrap() + 1;

    (min..max)
        .into_iter()
        .map(|position| compute_fuel(crabs, position))
        .min()
        .unwrap()
}

pub fn compute_fuel(positions: &Vec<i32>, position: i32) -> i32 {
    positions
        .iter()
        .map(|crab| (position - crab).abs())
        .fold(0, |acc: i32, distance: i32| {
            acc + calculate_increasing_costs(distance)
        })
}

pub fn calculate_increasing_costs(distance: i32) -> i32 {
    (0..(distance + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(cheapest_increasing_alignment(&input), 168);
    }
}
