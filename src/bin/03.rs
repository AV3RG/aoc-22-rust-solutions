use itertools::Itertools;
advent_of_code::solution!(3);

fn char_to_priority(c: &char) -> u16 {
    let n = *c as u16;
    match n {
        97..=122 => n-96,
        65..=90 => n-64+26,
        _ => panic!("Invalid character")
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    input.trim().split("\n").map(|line| {
        let split = line.trim().split_at(line.len() / 2);
        char_to_priority(&split.1.chars().find(|c| { split.0.chars().contains(c) }).unwrap())
    }).reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u16> {
    input.trim().split("\n").chunks(3)
        .into_iter().map(|mut chunk| {
            let s1 = chunk.next().unwrap();
            let s2 = chunk.next().unwrap();
            let s3 = chunk.next().unwrap();
            char_to_priority(&s1.chars().find(|c| { s2.chars().contains(c) && s3.chars().contains(c) }).unwrap())
    }).reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
