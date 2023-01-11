use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Assignment {
    first_assignee: (i32, i32),
    second_assignee: (i32, i32),
}

impl Assignment {
    fn has_contained_range(&self) -> bool {
        self.first_assignee.0 <= self.second_assignee.0
            && self.first_assignee.1 >= self.second_assignee.1
            || self.second_assignee.0 <= self.first_assignee.0
                && self.second_assignee.1 >= self.first_assignee.1
    }

    fn has_overlap(&self) -> bool {
        let first_assignee_range = self.first_assignee.0..=self.first_assignee.1;
        let second_assignee_range = self.second_assignee.0..=self.second_assignee.1;

        first_assignee_range.contains(&self.second_assignee.0)
            || first_assignee_range.contains(&self.second_assignee.1)
            || second_assignee_range.contains(&self.first_assignee.0)
            || second_assignee_range.contains(&self.first_assignee.1)
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Assignment> {
    input
        .split('\n')
        .map(|l| {
            let split = l.split_once(',').unwrap();
            let split_left = split.0.split_once('-').unwrap();
            let split_right = split.1.split_once('-').unwrap();

            Assignment {
                first_assignee: (
                    split_left.0.parse::<i32>().unwrap(),
                    split_left.1.parse::<i32>().unwrap(),
                ),
                second_assignee: (
                    split_right.0.parse::<i32>().unwrap(),
                    split_right.1.parse::<i32>().unwrap(),
                ),
            }
        })
        .collect::<Vec<Assignment>>()
}

#[aoc(day4, part1)]
pub fn part_one(assignments: &[Assignment]) -> i32 {
    assignments
        .iter()
        .filter(|a| a.has_contained_range())
        .count() as i32
}

#[aoc(day4, part2)]
pub fn part_two(assignments: &[Assignment]) -> i32 {
    assignments.iter().filter(|a| a.has_overlap()).count() as i32
}
