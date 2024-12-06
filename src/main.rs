use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
fn main() {
    // day_one_star_one();
    // day_one_star_two();
    // day_two_star_one();
    day_two_star_two();
}

fn day_one_star_one() {
    let mut left_list = [0];
    let mut right_list = [0];
    left_list.sort();
    right_list.sort();
    let mut total = 0;
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        let subtract: i32 = left - right;
        total = total + subtract.abs();
    }
    println!("{:?}", total);
    return;
}

fn create_hash_map(arr: &[i32]) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for item in arr {
        *map.entry(*item).or_insert(0) += 1;
    }
    return map;
}

fn day_one_star_two() {
    let left_list = [];
    let right_list = [];
    let left_map = create_hash_map(&left_list);
    let right_map = create_hash_map(&right_list);
    let mut total = 0;
    for (key, value) in left_map.into_iter() {
        if right_map.contains_key(&key) {
            total = total + (key * value * right_map[&key]);
        }
    }
    println!("{:?}", total);
    return;
}

fn is_safe(arr: &[i32]) -> bool {
    for (a, b) in arr.iter().zip(arr.iter().skip(1)) {
        if arr[0] < arr[1] {
            if b - a <=0 || b - a > 3 {
                return false;
            }
        } else if arr[0] > arr[1] {
            if a - b <=0 || a - b > 3 {
                return false;
            }
        } else {
            return false
        }
    }
    return true;
}

fn day_two_star_one() {
    let reports = [[]];
    let mut total_safe = 0;
    for report in reports {
        if is_safe(&report) {
            total_safe += 1;
        }
    }
    println!("{:?}", total_safe);
    return;
}

fn is_safe_drop(arr: &[i32]) -> Option<usize> {
    for (index, comparison) in arr.iter().zip(arr.iter().skip(1)).enumerate() {
        let a = comparison.0;
        let b: &i32 = comparison.1;
        if arr[0] < arr[1] {
            if b - a <=0 || b - a > 3 {
                return Some(index + 1);
            }
        } else if arr[0] > arr[1] {
            if a - b <=0 || a - b > 3 {
                return Some(index + 1);
            }
        } else {
            return Some(index + 1);
        }
    }
    return None;
}

fn drop_check(mut report: Vec<i32>) -> bool {
    let mut drop_idx = is_safe_drop(&report);
        if drop_idx.is_some() {
            if drop_idx.unwrap() == 1 {
                let drop_0 = is_safe_drop(&report[1..]);
                if drop_0.is_none() {
                    return true;
                } else {
                }
            }
            report.remove(drop_idx.unwrap());
            drop_idx = is_safe_drop(&report);
        }
        if drop_idx.is_none() {
            return true;
        } else {
            return false;
        }
}

fn day_two_star_two() {
    let file = File::open("day2part2.txt").ok().unwrap();
    let reader = BufReader::new(file);

    let mut total_safe = 0;

    for line in reader.lines() {
        let mut report: Vec<i32> = line.ok().unwrap().split_whitespace().filter_map(|word| word.parse::<i32>().ok()).collect();
        println!("{:?}", &report);
        if drop_check(report.clone()) {
            total_safe += 1;
        } else {
            report.reverse();
            if drop_check(report.clone()) {
                total_safe += 1;    
            }
        }
        
    }
    println!("{:?}", total_safe);
    return;
}
