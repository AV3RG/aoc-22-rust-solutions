use std::collections::{HashMap, HashSet};
use itertools::Itertools;
advent_of_code::solution!(9);

fn calc_tail_pos((head_row, head_col): (i16, i16), (tail_row, tail_col): (i16, i16)) -> (i16, i16) {
    let row_diff = head_row - tail_row;
    let col_diff = head_col - tail_col;

    if row_diff < 2 && row_diff > -2 && col_diff < 2 && col_diff > -2 {
        return (tail_row, tail_col);
    }

    (tail_row + row_diff.signum(), tail_col + col_diff.signum())
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited.insert(tail_pos);
    input.trim().lines().for_each(|line| {
        let (direction, amount_str) = line.split(" ").collect_tuple().unwrap();
        let amount = amount_str.parse::<i16>().unwrap();
        match direction {
            "U" => {
                for _ in 0..amount {
                    head_pos.1 -= 1;
                    tail_pos = calc_tail_pos(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            }
            "D" => {
                for _ in 0..amount {
                    head_pos.1 += 1;
                    tail_pos = calc_tail_pos(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            }
            "R" => {
                for _ in 0..amount {
                    head_pos.0 += 1;
                    tail_pos = calc_tail_pos(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            }
            "L" => {
                for _ in 0..amount {
                    head_pos.0 -= 1;
                    tail_pos = calc_tail_pos(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            }
            _ => panic!("Unknown direction"),
        }
    });
    Some(visited.len() as u16)
}

fn calc_last_node_pos((head_row, head_col): (i16, i16), followers: &mut HashMap<usize, (i16, i16)>)  {
    let mut last_leader_pos = (head_row, head_col);
    for (_, follower_pos) in followers.iter_mut() {
        last_leader_pos = calc_tail_pos(last_leader_pos, follower_pos.clone());
        *follower_pos = last_leader_pos;
    }
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut head_pos = (0, 0);
    let mut follower_pos: HashMap<usize, (i16, i16)> = HashMap::new();
    (1..=9).for_each(|i| { follower_pos.insert(i, (0, 0)); });
    visited.insert((0, 0));
    input.trim().lines().for_each(|line| {
        let (direction, amount_str) = line.split(" ").collect_tuple().unwrap();
        let amount = amount_str.parse::<i16>().unwrap();
        match direction {
            "U" => {
                for _ in 0..amount {
                    head_pos.1 -= 1;
                    calc_last_node_pos(head_pos, &mut follower_pos);
                    visited.insert(*follower_pos.get(&9).unwrap());
                }
            }
            "D" => {
                for _ in 0..amount {
                    head_pos.1 += 1;
                    calc_last_node_pos(head_pos, &mut follower_pos);
                    visited.insert(*follower_pos.get(&9).unwrap());
                }
            }
            "R" => {
                for _ in 0..amount {
                    head_pos.0 += 1;
                    calc_last_node_pos(head_pos, &mut follower_pos);
                    visited.insert(*follower_pos.get(&9).unwrap());
                }
            }
            "L" => {
                for _ in 0..amount {
                    head_pos.0 -= 1;
                    calc_last_node_pos(head_pos, &mut follower_pos);
                    visited.insert(*follower_pos.get(&9).unwrap());
                }
            }
            _ => panic!("Unknown direction"),
        }
    });
    Some(visited.len() as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
