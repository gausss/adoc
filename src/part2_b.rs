struct Movement {
    direction: String,
    amount: i32,
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn do_move(&mut self, movement: Movement) {
        match movement.direction.as_str() {
            "forward" => {
                self.horizontal += movement.amount;
                self.depth += self.aim * movement.amount
            }
            "up" => self.aim -= movement.amount,
            "down" => self.aim += movement.amount,
            _ => self.depth = self.depth,
        }
    }
}

pub fn compute_position_precise(movements: Vec<&str>) -> i32 {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for movement_string in movements {
        let movement_parts: Vec<&str> = movement_string.split_whitespace().into_iter().collect();
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

        assert_eq!(compute_position_precise(input_values.to_vec()), 900);
    }
    #[test]
    fn solve() {
        let input: Vec<&str> = include_str!("input/part2_input.txt")
            .split(",")
            .map(|input| input.trim())
            .collect();
        println!("Result 2.B: {}", compute_position_precise(input.to_vec()));
    }
}
