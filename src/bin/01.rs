advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.trim().split("\n\n").map(|group| {
            group.trim().split("\n").map(|f| f.parse::<u32>().unwrap())
                .reduce(|a, b| a + b).unwrap()
        }).max().unwrap()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = input.trim().split("\n\n").map(|group| {
        group.trim().split("\n").map(|f| f.parse::<u32>().unwrap())
            .reduce(|a, b| a + b).unwrap()
    }).collect::<Vec<u32>>();
    calories.sort();
    Some(
        calories[(calories.len() - 3)..(calories.len())].iter().map(|f| *f).reduce(|a, b| a + b).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
