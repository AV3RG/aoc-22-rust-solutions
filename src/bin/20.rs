advent_of_code::solution!(20);pub fn part_one(input: &str) -> Option<i64> {
    let mut numbers = input.trim().lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut ans = (0..numbers.len()).collect::<Vec<usize>>();
    for (i, &x) in numbers.iter().enumerate() {
        let pos = ans.iter().position(|&y| y == i).unwrap();
        ans.remove(pos);
        let new_pos = (pos as i64 + x).rem_euclid(ans.len() as i64) as usize;
        ans.insert(new_pos, i);
    }
    let original_zero = numbers.iter().position(|&x| x == 0).unwrap();
    let zero_pos = ans.iter().position(|&x| x == original_zero).unwrap();
    Some(
        [1000, 2000, 3000].iter().map(|i| numbers[ans[(zero_pos + i) % ans.len()]]).sum()
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let key = 811589153i64;
    let mut numbers = input.trim().lines().map(|line| line.parse::<i64>().unwrap() * key).collect::<Vec<i64>>();
    let mut ans = (0..numbers.len()).collect::<Vec<usize>>();
    for _ in 0..10 {
        for (i, &x) in numbers.iter().enumerate() {
            let pos = ans.iter().position(|&y| y == i).unwrap();
            ans.remove(pos);
            let new_pos = (pos as i64 + x).rem_euclid(ans.len() as i64) as usize;
            ans.insert(new_pos, i);
        }
    }
    let original_zero = numbers.iter().position(|&x| x == 0).unwrap();
    let zero_pos = ans.iter().position(|&x| x == original_zero).unwrap();
    Some(
        [1000, 2000, 3000].iter().map(|i| numbers[ans[(zero_pos + i) % ans.len()]]).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
