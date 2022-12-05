use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
pub struct Input {
    crates: HashMap<usize, Vec<String>>,
    instructions: Vec<Instruction>,
}

impl Input {
    pub fn run_instructions(&self, is_new_model: bool) -> String {
        let mut crates = self.crates.to_owned();
        let instructions = self.instructions.to_owned();

        instructions.iter().for_each(|instruction| {
            let Instruction { count, from, to } = instruction;

            let from = crates.get_mut(from).unwrap();

            let mut taken = from
                .iter()
                .rev()
                .take(*count)
                .map(|c| c.to_owned())
                .collect::<Vec<_>>();
            from.truncate(from.len() - count);

            if is_new_model {
                taken.reverse();
            }

            crates.get_mut(to).unwrap().append(&mut taken);
        });

        (1..=crates.len())
            .into_iter()
            .map(|key| crates.get(&key).unwrap().last().unwrap().to_owned())
            .collect::<Vec<_>>()
            .concat()
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    let lines = input.split('\n').collect::<Vec<_>>();
    let index = lines.iter().position(|l| l.is_empty()).unwrap();

    let state = &lines[0..index - 1];
    let instructions_raw = &lines[index + 1..];

    let state_regex = Regex::new(r"[a-zA-Z]|\s{4}").unwrap();

    let mut crates: HashMap<usize, Vec<String>> = HashMap::new();

    state.iter().rev().for_each(|s| {
        for (i, cap) in state_regex.captures_iter(s).enumerate() {
            let index = i + 1;
            let stack = crates.entry(index).or_default();

            if let Some(c) = cap.get(0) {
                let c: &str = c.into();
                if c != "    " {
                    stack.push(c.to_owned());
                }
            }
        }
    });

    let instructions_regex = Regex::new(r"\d+").unwrap();

    let instructions = instructions_raw
        .iter()
        .map(|instruction| {
            let capture = instructions_regex
                .captures_iter(instruction)
                .collect::<Vec<_>>();

            let c: &str = capture.get(0).unwrap().get(0).unwrap().into();
            let f: &str = capture.get(1).unwrap().get(0).unwrap().into();
            let t: &str = capture.get(2).unwrap().get(0).unwrap().into();

            Instruction {
                count: c.parse::<usize>().unwrap(),
                from: f.parse::<usize>().unwrap(),
                to: t.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Instruction>>();

    Input {
        crates,
        instructions,
    }
}

#[aoc(day5, part1)]
pub fn part_one(input: &Input) -> String {
    input.run_instructions(false)
}

#[aoc(day5, part2)]
pub fn part_two(input: &Input) -> String {
    input.run_instructions(true)
}
