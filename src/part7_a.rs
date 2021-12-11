pub fn cheapest_alignment(positions: &mut Vec<i32>) -> (i32, i32) {
    positions.sort();
    let mid = positions[positions.len() / 2];
    let distance = positions.iter().fold(0, |acc, x| acc + (mid - x).abs());

    return (mid, distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(cheapest_alignment(&mut input), (2, 37));
    }
}
