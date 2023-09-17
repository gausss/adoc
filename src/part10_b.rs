pub fn compute_risk_level(input: Vec<&str>) -> i64 {
    let missing_tokens: Vec<String> = input
        .iter()
        .filter_map(|x| {
            let result = parse_tokens(x);
            match result {
                Ok(buffer) => Some(buffer.iter().rev().collect::<String>()),
                _ => None,
            }
        })
        .collect();

    let mut scores: Vec<i64> = missing_tokens
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                })
                .fold(0, |total, c| total * 5 + c)
        })
        .collect();

    scores.sort();
    scores.get(scores.len() / 2).unwrap().clone()
}

fn parse_tokens(input: &str) -> Result<Vec<char>, char> {
    let mut check_tokens: Vec<char> = vec![];
    for token in input.chars() {
        if ['(', '[', '{', '<'].contains(&token) {
            check_tokens.push(match token {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => ' ',
            });
            continue;
        }

        let expected = check_tokens.pop().unwrap();
        if token != expected {
            return Err(token);
        }
    }
    Ok(check_tokens)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = include_str!("input/part10_test_input.txt").lines().collect();

        assert_eq!(compute_risk_level(input), 288957);
    }

    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part10_input.txt").lines().collect();

        println!("Result 10A: {}", compute_risk_level(input));
    }
}
