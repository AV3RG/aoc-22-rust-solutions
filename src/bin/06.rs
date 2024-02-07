advent_of_code::solution!(6);

fn solve(input: &str, size: u16) -> Option<u16> {
    let mut state: [i16; 26] = [-1; 26];
    let mut left = 0;
    let mut right = 0;
    for (index, c) in input.char_indices() {
        if right - left == size { return Some(right); }
        right += 1;
        if state[c as usize - 97] < left as i16 {
            state[c as usize - 97] = index as i16;
        } else {
            left = (state[c as usize - 97] as usize + 1) as u16;
            state[c as usize - 97] = index as i16;
        }
    }
    panic!("No solution found");
}

pub fn part_one(input: &str) -> Option<u16> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u16> {
    solve(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
