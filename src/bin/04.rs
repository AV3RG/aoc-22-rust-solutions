use regex::{Captures, Regex};
advent_of_code::solution!(4);

fn captured_to_int(group_name: &str, captures: &Captures) -> u16 {
    captures[group_name].parse::<u16>().unwrap()
}

fn range_contains(inner: (u16, u16), outer: (u16, u16)) -> bool {
    outer.0 <= inner.0 && outer.1 >= inner.1
}

fn range_no_overlap(r1: (u16, u16), r2: (u16, u16)) -> bool {
    r1.1 < r2.0 || r1.0 > r2.1
}

pub fn part_one(input: &str) -> Option<u16> {
    let regex = Regex::new(r"(?P<l1>\d+)-(?P<u1>\d+),(?P<l2>\d+)-(?P<u2>\d+)").unwrap();
    Some(
        input.trim().split("\n").filter(|line| {
            let captured = regex.captures(line.trim()).unwrap();
            let r1 = (captured_to_int("l1", &captured), captured_to_int("u1", &captured));
            let r2 = (captured_to_int("l2", &captured), captured_to_int("u2", &captured));
            range_contains(r1, r2) || range_contains(r2, r1)
        }).count() as u16
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    let regex = Regex::new(r"(?P<l1>\d+)-(?P<u1>\d+),(?P<l2>\d+)-(?P<u2>\d+)").unwrap();
    Some(
        input.trim().split("\n").filter(|line| {
            let captured = regex.captures(line.trim()).unwrap();
            let r1 = (captured_to_int("l1", &captured), captured_to_int("u1", &captured));
            let r2 = (captured_to_int("l2", &captured), captured_to_int("u2", &captured));
            !range_no_overlap(r1, r2)
        }).count() as u16
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
