use std::{path, vec};

pub fn compute_possible_paths(input: Vec<&str>) -> usize {
    let edges: Vec<(&str, &str)> = input
        .iter()
        .map(|line| {
            let parts = line.split_once("-").unwrap();
            if parts.0 == "start" || parts.1 == "end" {
                return vec![(parts.0, parts.1)];
            } else {
                return vec![(parts.0, parts.1), (parts.1, parts.0)];
            }
        })
        .flatten()
        .collect();

    let mut paths: Vec<Vec<String>> = edges
        .iter()
        .filter(|edge| edge.0 == "start")
        .map(|edge| vec![edge.0.to_string(), edge.1.to_string()])
        .collect();

    let mut paths_complete: Vec<Vec<String>> = vec![];
    loop {
        let mut paths_tmp = vec![];
        for path in &paths {
            if path.last().unwrap() == "end" {
                paths_complete.push(path.clone());
            }

            paths_tmp.extend(traverse(&edges, &path));
        }

        if paths_tmp.len() == 0 {
            break;
        } else {
            paths = paths_tmp.clone();
        }
    }

    println!("{:?}", paths_complete);

    paths_complete.iter().count()
}

pub fn traverse(edges: &Vec<(&str, &str)>, path: &Vec<String>) -> Vec<Vec<String>> {
    let start = path.last().unwrap();
    let extended_paths: Vec<Vec<String>> = edges
        .iter()
        .filter(|edge| &edge.0 == start)
        .filter(|edge| {
            edge.1 == "end"
                || edge.1.to_uppercase() == edge.1
                || !path.contains(&edge.1.to_string())
        })
        .map(|edge| {
            let mut path_ext = vec![];
            for point in path {
                path_ext.push(point.to_owned());
            }
            path_ext.push(edge.1.to_string());
            path_ext
        })
        .map(|path| path.iter().map(|a| a.to_string()).collect())
        .collect();

    extended_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = include_str!("input/part12_test_input.txt")
            .lines()
            .collect();

        assert_eq!(compute_possible_paths(input), 10);
    }

    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part12_input.txt").lines().collect();

        println!("Result 12A: {}", compute_possible_paths(input));
    }
}
