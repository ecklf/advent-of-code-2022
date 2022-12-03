use aoc_runner_derive::{aoc, aoc_generator};

fn get_priority(c: char) -> i32 {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (letters.find(c).unwrap() as i32) + 1
}

#[derive(Debug)]
pub struct Compartment {
    full: String,
    one: String,
    two: String,
}

impl Compartment {
    fn duplicate_priority(&self) -> i32 {
        let entries = self
            .one
            .chars()
            .filter_map(|c| self.two.find(c).map(|_| c))
            .collect::<Vec<_>>();

        get_priority(*entries.first().unwrap())
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Compartment> {
    input
        .split('\n')
        .map(|l| {
            let split = l.split_at(l.len() / 2);
            Compartment {
                full: l.to_owned(),
                one: split.0.to_owned(),
                two: split.1.to_owned(),
            }
        })
        .collect::<Vec<Compartment>>()
}

#[aoc(day3, part1)]
pub fn part_one(compartments: &[Compartment]) -> i32 {
    compartments
        .iter()
        .map(|c| c.duplicate_priority())
        .sum::<i32>()
}

#[aoc(day3, part2)]
pub fn part_two(compartments: &[Compartment]) -> i32 {
    let res: i32 = compartments
        .chunks(3)
        .map(|w| {
            return w[0]
                .full
                .chars()
                .filter(|c| w[1].full.find(*c).is_some() && w[2].full.find(*c).is_some())
                .collect::<Vec<char>>();
        })
        .map(|v| get_priority(v[0]))
        .sum();

    res
}
