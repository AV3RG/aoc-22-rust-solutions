use std::collections::HashSet;
use std::mem::swap;
use std::ops::{Add, Sub};
use itertools::Itertools;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut filled: HashSet<(u32, u32)> = HashSet::new();
    let mut highest_y = 0;
    input.trim().lines().for_each(|line| {
        let coords = line.split(" -> ")
            .map(|coordinate|
                coordinate.split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect_tuple().unwrap()
            )
            .collect::<Vec<(u32, u32)>>();
        for x in 0..coords.len().sub(1) {
            let (mut start_x, mut start_y) = coords[x];
            let (mut end_x, mut end_y) = coords[x+1];
            if end_y > highest_y { highest_y = end_y; }
            if start_y > highest_y { highest_y = start_y; }
            if start_x > end_x { swap(&mut start_x, &mut end_x) }
            if start_y > end_y { swap(&mut start_y, &mut end_y) }
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    filled.insert((x, y));
                }
            }
        }
    });
    let mut count = 0;
    loop {
        let mut sand = (500u32, 0u32);
        let mut edge_reached = false;
        loop {
            if sand.1 > highest_y { edge_reached = true; break; }
            if !filled.contains(&(sand.0, sand.1.add(1))) { sand.1 += 1 }
            else if !filled.contains(&(sand.0.sub(1), sand.1.add(1))) { sand.0 -= 1; sand.1 += 1 }
            else if !filled.contains(&(sand.0.add(1), sand.1.add(1))) { sand.0 += 1; sand.1 += 1 }
            else { break }
        }
        if edge_reached { break }
        count += 1;
        filled.insert(sand);
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut filled: HashSet<(u32, u32)> = HashSet::new();
    let mut highest_y = 0;
    input.trim().lines().for_each(|line| {
        let coords = line.split(" -> ")
            .map(|coordinate|
                coordinate.split(",")
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect_tuple().unwrap()
            )
            .collect::<Vec<(u32, u32)>>();
        for x in 0..coords.len().sub(1) {
            let (mut start_x, mut start_y) = coords[x];
            let (mut end_x, mut end_y) = coords[x+1];
            if end_y > highest_y { highest_y = end_y; }
            if start_y > highest_y { highest_y = start_y; }
            if start_x > end_x { swap(&mut start_x, &mut end_x) }
            if start_y > end_y { swap(&mut start_y, &mut end_y) }
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    filled.insert((x, y));
                }
            }
        }
    });
    let mut count = 0;
    loop {
        let mut sand = (500u32, 0u32);
        loop {
            if sand.1 > highest_y { break; }
            if !filled.contains(&(sand.0, sand.1.add(1))) { sand.1 += 1 }
            else if !filled.contains(&(sand.0.sub(1), sand.1.add(1))) { sand.0 -= 1; sand.1 += 1 }
            else if !filled.contains(&(sand.0.add(1), sand.1.add(1))) { sand.0 += 1; sand.1 += 1 }
            else { break }
        }
        count += 1;
        filled.insert(sand);
        if sand == (500, 0) { break }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(93));
    }
}
