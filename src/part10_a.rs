pub fn compute_risk_level(input: Vec<&str>) -> i32 {
    input.iter()
        .map(|x| match parse_tokens(x) {
            Ok(_) => 0,
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => 0,
        })
        .sum()
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

        assert_eq!(compute_risk_level(input), 26397);
    }

    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part10_input.txt").lines().collect();

        println!("Result 10A: {}", compute_risk_level(input));
    }
}
