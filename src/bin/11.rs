use std::cell::RefCell;
use std::ops::{Div, Rem};
use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(11);

struct Monkey {
    current: Vec<u64>,
    operation: char,
    operand: u64,
    is_operand_self: bool,
    div_check: u64,
    true_monkey: u8,
    false_monkey: u8,
    inspected: u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let regex = Regex::new(r"Monkey \d+:\n\s+Starting items: (?P<starting>.+)\n\s+Operation: new = old (?P<operation>.) (?P<operand>.+)\n\s+Test: divisible by (?P<div_check>\d+)\n\s+If true: throw to monkey (?P<true_monkey>\d+)\n\s+If false: throw to monkey (?P<false_monkey>\d+)")
        .unwrap();
    let monkeys: Vec<RefCell<Monkey>> = input.trim().split("\n\n").map(|monkey_data| {
        let captured = regex.captures(monkey_data).unwrap();
        let starting: Vec<u64> = captured["starting"].split(", ").map(|s| s.parse::<u64>().unwrap()).collect();
        let operation = &captured["operation"].chars().next().unwrap();
        let operand = &captured["operand"];
        let div_check = captured["div_check"].parse::<u64>().unwrap();
        let true_monkey = captured["true_monkey"].parse::<u8>().unwrap();
        let false_monkey = captured["false_monkey"].parse::<u8>().unwrap();
        let is_operand_self = operand == "old";
        let operand = if is_operand_self { 0 } else { operand.parse::<u64>().unwrap() };
        RefCell::new(
            Monkey {
                current: starting,
                operation: *operation,
                operand,
                is_operand_self,
                div_check,
                true_monkey,
                false_monkey,
                inspected: 0,
            }
        )
    }).collect();
    (0..20).for_each(|_| {
        monkeys.iter().for_each(|monkey_ref| {
            let mut monkey = monkey_ref.borrow_mut();
            while monkey.current.len() > 0 {
                let item_to_check = monkey.current.pop().unwrap();
                monkey.inspected += 1;
                let use_operand = if monkey.is_operand_self { item_to_check } else { monkey.operand };
                let new_worry = match monkey.operation {
                    '+' => item_to_check + use_operand,
                    '-' => item_to_check - use_operand,
                    '*' => item_to_check * use_operand,
                    '/' => item_to_check / use_operand,
                    _ => panic!("Invalid operation")
                }.div(3);
                if new_worry.rem(monkey.div_check) == 0 {
                    monkeys[monkey.true_monkey as usize].borrow_mut().current.push(new_worry);
                } else {
                    monkeys[monkey.false_monkey as usize].borrow_mut().current.push(new_worry);
                }
            }
        });
    });
    monkeys.iter().map(|monkey| monkey.borrow().inspected)
        .sorted_by(|a, b| { b.cmp(a) })
        .take(2).reduce(|a, b| a * b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(r"Monkey \d+:\n\s+Starting items: (?P<starting>.+)\n\s+Operation: new = old (?P<operation>.) (?P<operand>.+)\n\s+Test: divisible by (?P<div_check>\d+)\n\s+If true: throw to monkey (?P<true_monkey>\d+)\n\s+If false: throw to monkey (?P<false_monkey>\d+)")
        .unwrap();
    let mut stress_mod: u64 = 1;
    let monkeys: Vec<RefCell<Monkey>> = input.trim().split("\n\n").map(|monkey_data| {
        let captured = regex.captures(monkey_data).unwrap();
        let starting: Vec<u64> = captured["starting"].split(", ").map(|s| s.parse::<u64>().unwrap()).collect();
        let operation = &captured["operation"].chars().next().unwrap();
        let operand = &captured["operand"];
        let div_check = captured["div_check"].parse::<u64>().unwrap();
        let true_monkey = captured["true_monkey"].parse::<u8>().unwrap();
        let false_monkey = captured["false_monkey"].parse::<u8>().unwrap();
        let is_operand_self = operand == "old";
        let operand = if is_operand_self { 0 } else { operand.parse::<u64>().unwrap() };
        stress_mod *= div_check;
        RefCell::new(
            Monkey {
                current: starting,
                operation: *operation,
                operand,
                is_operand_self,
                div_check,
                true_monkey,
                false_monkey,
                inspected: 0,
            }
        )
    }).collect();
    (0..10000).for_each(|_| {
        monkeys.iter().for_each(|monkey_ref| {
            let mut monkey = monkey_ref.borrow_mut();
            while monkey.current.len() > 0 {
                let item_to_check = monkey.current.pop().unwrap();
                monkey.inspected += 1;
                let use_operand = if monkey.is_operand_self { item_to_check } else { monkey.operand };
                let new_worry = match monkey.operation {
                    '+' => item_to_check + use_operand,
                    '-' => item_to_check - use_operand,
                    '*' => item_to_check * use_operand,
                    '/' => item_to_check / use_operand,
                    _ => panic!("Invalid operation")
                }.rem(stress_mod);
                if new_worry.rem(monkey.div_check) == 0 {
                    monkeys[monkey.true_monkey as usize].borrow_mut().current.push(new_worry);
                } else {
                    monkeys[monkey.false_monkey as usize].borrow_mut().current.push(new_worry);
                }
            }
        });
    });
    monkeys.iter().map(|monkey| monkey.borrow().inspected)
        .sorted_by(|a, b| { b.cmp(a) })
        .take(2).reduce(|a, b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2713310158));
    }
}
