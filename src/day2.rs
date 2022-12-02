use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub struct Strategy {
    opponent_shape: Shape,
    own_shape: Shape,
}

impl Strategy {
    fn get_shape_score(&self, shape: &Shape) -> i32 {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn get_game_socre(&self, own_shape: &Shape, opponent_shape: &Shape) -> i32 {
        match own_shape {
            Shape::Rock => match opponent_shape {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match opponent_shape {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match opponent_shape {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        }
    }

    fn calc_score_part1(&self) -> i32 {
        let shape_score = self.get_shape_score(&self.own_shape);
        let game_score = self.get_game_socre(&self.own_shape, &self.opponent_shape);
        shape_score + game_score
    }

    fn calc_score_part2(&self) -> i32 {
        let own_shape = match self.own_shape {
            // You need to lose
            Shape::Rock => match self.opponent_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            // You need to draw
            Shape::Paper => match self.opponent_shape {
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            },
            // You need to win
            Shape::Scissors => match self.opponent_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };
        let shape_score = self.get_shape_score(&own_shape);
        let game_score = self.get_game_socre(&own_shape, &self.opponent_shape);
        shape_score + game_score
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Strategy> {
    input
        .split('\n')
        .map(|line| {
            let opponent = line.chars().next().unwrap();
            let own = line.chars().nth(2).unwrap();

            let opponent_shape = match opponent {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissors,
                _ => unreachable!(),
            };

            let own_shape = match own {
                'X' => Shape::Rock,
                'Y' => Shape::Paper,
                'Z' => Shape::Scissors,
                _ => unreachable!(),
            };

            Strategy {
                opponent_shape,
                own_shape,
            }
        })
        .collect::<Vec<Strategy>>()
}

#[aoc(day2, part1)]
pub fn part_one(strategies: &[Strategy]) -> i32 {
    strategies.iter().map(|s| s.calc_score_part1()).sum::<i32>()
}

#[aoc(day2, part2)]
pub fn part_two(strategies: &[Strategy]) -> i32 {
    strategies.iter().map(|s| s.calc_score_part2()).sum::<i32>()
}
