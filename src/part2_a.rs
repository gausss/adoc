struct Movement {
    direction: String,
    amount: i32,
}

struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    pub fn do_move(&mut self, movement: Movement) {
        match movement.direction.as_str() {
            "forward" => self.horizontal += movement.amount,
            "up" => self.depth -= movement.amount,
            "down" => self.depth += movement.amount,
            _ => self.depth = self.depth,
        }
    }
}

pub fn compute_position(movements: Vec<&str>) -> i32 {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for movement_string in movements {
        let movement_parts: Vec<&str> = movement_string
            .split_whitespace()
            .into_iter()
            .collect();
        let movement = Movement {
            direction: movement_parts[0].to_string(),
            amount: movement_parts[1].parse().unwrap(),
        };

        position.do_move(movement);
    }

    return position.horizontal * position.depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_values = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        assert_eq!(compute_position(input_values.to_vec()), 150);
    }
}
