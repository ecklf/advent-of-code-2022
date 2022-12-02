use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    input.split('\n').fold(Vec::<i32>::new(), |mut acc, x| {
        match x.parse::<i32>() {
            Ok(val) => {
                let length = acc.len();
                if length == 0 {
                    acc.push(val);
                } else {
                    acc[length - 1] += val;
                }
            }
            Err(_b) => {
                acc.push(0);
            }
        };
        acc
    })
}

#[aoc(day1, part1)]
pub fn part_one(calories: &[i32]) -> i32 {
    let mut result = calories.to_owned();
    result.sort();
    *result.last().unwrap()
}

#[aoc(day1, part2)]
pub fn part_two(calories: &[i32]) -> i32 {
    let mut result = calories.to_owned();
    result.sort();
    result.iter().rev().take(3).sum::<i32>()
}
