struct Fish {
    time_to_breed: i32,
}

impl Fish {
    pub fn advance(&mut self) -> bool {
        if self.time_to_breed == 0 {
            self.time_to_breed = 6;
            return true;
        }

        self.time_to_breed -= 1;
        return false;
    }
}

pub fn model_population(initial_population: &Vec<i32>, passing_days: i32) -> usize {
    let mut modeled_population: Vec<Fish> = initial_population
        .iter()
        .map(|lifetime| Fish {
            time_to_breed: lifetime.to_owned(),
        })
        .collect();

    for _day in 0..passing_days {
        let mut newborns = 0;
        for fish in &mut modeled_population {
            if fish.advance() {
                newborns += 1;
            }
        }

        for _newborn in 0..newborns {
            modeled_population.push(Fish {
                time_to_breed: 8
            });
        }
    }
    return modeled_population.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<i32> = vec![3, 4, 3, 1, 2];

        assert_eq!(model_population(&input, 18), 26);
        assert_eq!(model_population(&input, 80), 5934);
    }
}
