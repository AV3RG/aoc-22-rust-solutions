use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::{Add, Div};
use itertools::Itertools;
use regex::{Captures, Regex};
advent_of_code::solution!(5);

fn captured_to_int(group_name: &str, captures: &Captures) -> u16 {
    captures[group_name].parse::<u16>().unwrap()
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks_str, moves) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks: Vec<RefCell<VecDeque<char>>> = Vec::new();
    let lines = stacks_str.lines().rev();
    for _ in 0..lines.clone().take(1).next().unwrap().len().add(2).div(4) {
        stacks.push(RefCell::new(VecDeque::new()));
    };
    lines.skip(1)
        .map(|line| line.chars().chunks(4))
        .for_each(|line_chunked| {
            line_chunked.into_iter().map(|chunk_charred| {
                chunk_charred.skip(1).next().unwrap()
            }).enumerate().for_each(|(index, char)| {
                if char == ' ' { return; }
                stacks.get(index).unwrap().borrow_mut().push_front(char);
            });
        });

    let regex = Regex::new(r"move (?P<amount>\d+) from (?P<ini>\d+) to (?P<final>\d+)").unwrap();
    moves.lines().map(|move_line| {
        let captured = regex.captures(move_line).unwrap();
        (
            captured_to_int("amount", &captured),
            captured_to_int("ini", &captured) - 1,
            captured_to_int("final", &captured) - 1
        )
    }).for_each(|(amount, start, dest)| {
        let mut start_stack = stacks.get(start as usize).unwrap().try_borrow_mut().unwrap();
        let mut dest_stack = stacks.get(dest as usize).unwrap().try_borrow_mut().unwrap();
        for _ in 0..amount {
            dest_stack.push_front(start_stack.pop_front().unwrap());
        }
    });
    stacks.iter().map(|stack| {
        stack.try_borrow_mut().unwrap().pop_front().unwrap().to_string()
    }).reduce(|a, b| a+ &*b)

}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks_str, moves) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks: Vec<RefCell<VecDeque<char>>> = Vec::new();
    let lines = stacks_str.lines().rev();
    for _ in 0..lines.clone().take(1).next().unwrap().len().add(2).div(4) {
        stacks.push(RefCell::new(VecDeque::new()));
    };
    lines.skip(1)
        .map(|line| line.chars().chunks(4))
        .for_each(|line_chunked| {
            line_chunked.into_iter().map(|chunk_charred| {
                chunk_charred.skip(1).next().unwrap()
            }).enumerate().for_each(|(index, char)| {
                if char == ' ' { return; }
                stacks.get(index).unwrap().borrow_mut().push_front(char);
            });
        });

    let regex = Regex::new(r"move (?P<amount>\d+) from (?P<ini>\d+) to (?P<final>\d+)").unwrap();
    moves.lines().map(|move_line| {
        let captured = regex.captures(move_line).unwrap();
        (
            captured_to_int("amount", &captured),
            captured_to_int("ini", &captured) - 1,
            captured_to_int("final", &captured) - 1
        )
    }).for_each(|(amount, start, dest)| {
        let mut start_stack = stacks.get(start as usize).unwrap().try_borrow_mut().unwrap();
        let mut dest_stack = stacks.get(dest as usize).unwrap().try_borrow_mut().unwrap();
        start_stack.drain(0..(amount as usize)).rev().for_each(|c| {
            dest_stack.push_front(c);
        });
    });
    stacks.iter().map(|stack| {
        stack.try_borrow_mut().unwrap().pop_front().unwrap().to_string()
    }).reduce(|a, b| a+ &*b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("MCD".to_string()));
    }
}
