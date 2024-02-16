use std::collections::HashSet;
use std::ops::{Add, Mul, Sub};
use itertools::Itertools;
use regex::{Captures, Regex};
advent_of_code::solution!(15);

fn captured_to_int(group_name: &str, captures: &Captures) -> i64 {
    captures[group_name].parse::<i64>().unwrap()
}

fn manhattan_distance(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    return pos1.0.abs_diff(pos2.0).add(pos1.1.abs_diff(pos2.1)) as i64
}

fn range_collide(r1: &(i64, i64), r2: &(i64, i64)) -> bool {
    return r1.0 <= r2.1 && r1.1 >= r2.0
}

pub fn part_one(input: &str) -> Option<i64> {
    let regex = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)")
        .unwrap();
    let target_y = 2000000i64;
    // let target_y = 10i64;
    let mut beacons_in_target: HashSet<i64> = HashSet::new();
    let sensors_data = input.trim().lines().map(|line| {
        let captured = regex.captures(line).unwrap();
        let sensor = (
            captured_to_int("sensor_x", &captured),
            captured_to_int("sensor_y", &captured)
        );
        let beacon = (
            captured_to_int("beacon_x", &captured),
            captured_to_int("beacon_y", &captured),
        );
        if beacon.1 == target_y { beacons_in_target.insert(beacon.0); }
        (sensor, manhattan_distance(sensor, beacon))
    }).collect::<Vec<((i64, i64), i64)>>();
    let mut filled_ranges: Vec<(i64, i64)> = Vec::new();
    for sensor in sensors_data {
        let distance_from_target_y = target_y.abs_diff(sensor.0.1) as i64;
        if distance_from_target_y > sensor.1 { continue; }
        let left_distance = sensor.1.sub(distance_from_target_y);
        let new_range = (sensor.0.0.sub(left_distance), sensor.0.0.add(left_distance));
        filled_ranges.push(new_range);
    }
    filled_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    let mut current_range = filled_ranges[0];
    for range in filled_ranges.iter().skip(1) {
        if range_collide(&current_range, range) || range_collide(range, &current_range) {
            current_range = (current_range.0.min(range.0), current_range.1.max(range.1));
        } else {
            merged_ranges.push(current_range);
            current_range = *range;
        }
    }
    merged_ranges.push(current_range);
    let subtract = beacons_in_target.iter().filter(|beacon| {
        filled_ranges.iter().any(|range| range_collide(&range, &(**beacon, **beacon)))
    }).count();
    Some(
        merged_ranges.iter().map(|range| range.1.sub(range.0).add(1)).sum::<i64>().sub(subtract as i64)
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let regex = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)")
        .unwrap();
    // let max_search = 20i64;
    let min_search = 0i64;
    let max_search = 4000000i64;
    let sensors_data = input.trim().lines().map(|line| {
        let captured = regex.captures(line).unwrap();
        let sensor = (
            captured_to_int("sensor_x", &captured),
            captured_to_int("sensor_y", &captured)
        );
        let beacon = (
            captured_to_int("beacon_x", &captured),
            captured_to_int("beacon_y", &captured),
        );
        (sensor, manhattan_distance(sensor, beacon))
    }).collect::<Vec<((i64, i64), i64)>>();
    for target_y in min_search..=max_search {
        let mut filled_ranges: Vec<(i64, i64)> = Vec::new();
        for sensor in sensors_data.clone() {
            let distance_from_target_y = target_y.abs_diff(sensor.0.1) as i64;
            if distance_from_target_y > sensor.1 { continue; }
            let left_distance = sensor.1.sub(distance_from_target_y);
            let new_range = (sensor.0.0.sub(left_distance), sensor.0.0.add(left_distance));
            filled_ranges.push(new_range);
        }
        filled_ranges.sort_by(|a, b| a.0.cmp(&b.0));
        let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
        let mut current_range = filled_ranges[0];
        for range in filled_ranges.iter().skip(1) {
            if range_collide(&current_range, range) {
                current_range = (current_range.0.min(range.0), current_range.1.max(range.1));
            } else {
                merged_ranges.push(current_range);
                current_range = *range;
            }
        }
        merged_ranges.push(current_range);
        let covered = merged_ranges.iter().any(|range| range.0 <= 0 && range.1 >= 4000000);
        if !covered {
            return Some(merged_ranges.iter().next().unwrap().1.add(1).mul(4000000).add(target_y))
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(56000011));
    }
}
