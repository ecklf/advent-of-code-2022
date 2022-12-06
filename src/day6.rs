use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> String {
    input.to_owned()
}

fn process_signal(signal: &str, marker_length: usize) -> i32 {
    let mut cursor: i32 = 0;
    let mut queue: VecDeque<char> = VecDeque::new();

    let take_marker_length = |queue: VecDeque<char>| -> Vec<char> {
        queue
            .iter()
            .rev()
            .take(marker_length)
            .map(|f| f.to_owned())
            .collect::<Vec<_>>()
    };

    for c in signal.chars() {
        // Skip if not long enough
        if queue.len() < marker_length {
            queue.push_back(c);
            cursor += 1;
        } else {
            // Ensure last n characters are all unique
            let mut marker_chars = take_marker_length(queue.clone());
            marker_chars.sort();
            marker_chars.dedup();

            if marker_chars.len() == marker_length {
                break;
            }

            queue.push_back(c);
            cursor += 1;
        }
    }

    cursor
}

#[aoc(day6, part1)]
pub fn part_one(signal: &str) -> i32 {
    const MARKER_LENGTH: usize = 4;
    process_signal(signal, MARKER_LENGTH)
}

#[aoc(day6, part2)]
pub fn part_two(signal: &str) -> i32 {
    const MARKER_LENGTH: usize = 14;
    process_signal(signal, MARKER_LENGTH)
}
