use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufReader, BufRead};
use regex::Regex;
fn main() {
    // day_one_star_one();
    // day_one_star_two();
    // day_two_star_one();
    // day_two_star_two();
    // day_three_star_one();
    // day_three_star_two();
    // day_four_star_one();
    // day_four_star_two();
    // day_five_star_one();
    // day_five_star_two();
    // day_six_star_one();
    day_six_star_two();
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

fn day_three_star_one() {
    let file = File::open("day3.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let mut total = 0;

    for line in reader.lines() {
        let in_str = line.ok().unwrap();
        for groups in pattern.captures_iter(&in_str) {
            let left: i32 = groups.get(1).unwrap().as_str().parse().expect("Invalid number");
            let right: i32 = groups.get(2).unwrap().as_str().parse().expect("Invalid Number");
            total = total + (left * right);
        }
    }

    println!("{:?}", total);
}

fn day_three_star_two() {
    let file = File::open("day3.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").expect("Invalid regex");
    let mut total = 0;

    let mut matches = Vec::new();

    for line in reader.lines() {
        let in_str = line.ok().unwrap();
        
        for mat in pattern.find_iter(&in_str) {
            matches.push(mat.as_str().to_string());
        }
    }

    let mut enabled = true;

    for eval in matches {
        if eval.eq_ignore_ascii_case("do()") {
            enabled = true
        } else if eval.eq_ignore_ascii_case("don't()") {
            enabled = false
        } else {
            if enabled == true {
                let num_pattern = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
                for groups in num_pattern.captures_iter(&eval) {
                    let left: i32 = groups.get(1).unwrap().as_str().parse().expect("Invalid number");
                    let right: i32 = groups.get(2).unwrap().as_str().parse().expect("Invalid Number");
                    total = total + (left * right);
                }
            }
        }
    }
    println!("{:?}", total);


}

fn day_four_star_one() {
    let file = File::open("day4.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    let valid_str = vec!['X', 'M', 'A', 'S'];

    let lines: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    for (line_idx, line) in lines.clone().into_iter().enumerate() {
        for (char_idx, ch) in line.clone().into_iter().enumerate() {
            if ch == 'X' || ch == 'S' {
                let mut e: Vec<char> = vec![];
                let mut se: Vec<char> = vec![];
                let mut s: Vec<char> = vec![];
                let mut sw: Vec<char> = vec![];

                if char_idx < line.len() - 3 {
                    e.insert(0, *lines.get(line_idx).unwrap().get(char_idx).unwrap());
                    e.insert(1, *lines.get(line_idx).unwrap().get(char_idx + 1).unwrap());
                    e.insert(2, *lines.get(line_idx).unwrap().get(char_idx + 2).unwrap());
                    e.insert(3, *lines.get(line_idx).unwrap().get(char_idx + 3).unwrap());
                }

                if char_idx < line.len() - 3 && line_idx < lines.len() - 3 {
                    se.insert(0, *lines.get(line_idx).unwrap().get(char_idx).unwrap());
                    se.insert(1, *lines.get(line_idx + 1).unwrap().get(char_idx + 1).unwrap());
                    se.insert(2, *lines.get(line_idx + 2).unwrap().get(char_idx + 2).unwrap());
                    se.insert(3, *lines.get(line_idx + 3).unwrap().get(char_idx + 3).unwrap());
                }

                if line_idx < lines.len() - 3 {
                    s.insert(0, *lines.get(line_idx).unwrap().get(char_idx).unwrap());
                    s.insert(1, *lines.get(line_idx + 1).unwrap().get(char_idx).unwrap());
                    s.insert(2, *lines.get(line_idx + 2).unwrap().get(char_idx).unwrap());
                    s.insert(3, *lines.get(line_idx + 3).unwrap().get(char_idx).unwrap());
                }

                if char_idx >= 3 && line_idx < lines.len() - 3 {
                    sw.insert(0, *lines.get(line_idx).unwrap().get(char_idx).unwrap());
                    sw.insert(1, *lines.get(line_idx + 1).unwrap().get(char_idx - 1).unwrap());
                    sw.insert(2, *lines.get(line_idx + 2).unwrap().get(char_idx - 2).unwrap());
                    sw.insert(3, *lines.get(line_idx + 3).unwrap().get(char_idx - 3).unwrap());
                }

                if e == valid_str || e.iter().rev().eq(valid_str.iter()) {
                    total += 1;
                }
                if se == valid_str || se.iter().rev().eq(valid_str.iter()) {
                    total += 1;
                }
                if s == valid_str || s.iter().rev().eq(valid_str.iter()) {
                    total += 1;
                }
                if sw == valid_str || sw.iter().rev().eq(valid_str.iter()) {
                    total += 1;
                }

            }
        }
    }
    println!("{:?}", total);
}


fn day_four_star_two() {
    let file = File::open("day4.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    let valid_str = vec!['M', 'A', 'S'];

    let lines: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    for (line_idx, line) in lines.clone().into_iter().enumerate() {
        for (char_idx, ch) in line.clone().into_iter().enumerate() {
            if ch == 'A' {
                let mut backslash: Vec<char> = vec![];
                let mut forwardslash: Vec<char> = vec![];

                if char_idx > 0 && char_idx < line.len() - 1 && line_idx > 0 && line_idx < lines.len() - 1 {
                    backslash.insert(0, *lines.get(line_idx - 1).unwrap().get(char_idx - 1).unwrap());
                    backslash.insert(1, *lines.get(line_idx).unwrap().get(char_idx).unwrap());
                    backslash.insert(2, *lines.get(line_idx + 1).unwrap().get(char_idx + 1).unwrap());

                    forwardslash.insert(0, *lines.get(line_idx + 1).unwrap().get(char_idx - 1).unwrap());
                    forwardslash.insert(1, *lines.get(line_idx).unwrap().get(char_idx).unwrap());
                    forwardslash.insert(2, *lines.get(line_idx - 1).unwrap().get(char_idx + 1).unwrap());
                }

                if (backslash == valid_str || backslash.iter().rev().eq(valid_str.iter())) && (forwardslash == valid_str || forwardslash.iter().rev().eq(valid_str.iter())) {
                    total += 1;
                }

            }
        }
    }
    println!("{:?}", total);
}

fn is_valid_position(number: i32, must_be_before: &Vec<i32>, report: &Vec<&str>) -> bool {
    let last_idx = report.iter().rposition(|&x| x.parse::<i32>().ok().unwrap() == number).unwrap();
    for rule in must_be_before {
        let first_idx = report.iter().position(|&x| x == rule.to_string());
        if first_idx.is_some() && last_idx > first_idx.unwrap() {
            return false;
        }
    }
    return true;
}

fn process_report(report: &Vec<&str>, rules_map: &HashMap<i32, Vec<i32>>) -> bool {
    for item in report {
        let number = item.parse::<i32>().ok().unwrap();
        if rules_map.contains_key(&number) {
            if !is_valid_position(number, &rules_map[&number], &report) {
                return false;
            }
        }
    }
    return true;
}

fn day_five_star_one() {
    let file = File::open("day5.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in reader.lines() {
        let in_str = line.ok().unwrap();
        if in_str.contains("|") {
            let new_rule: Vec<&str> = in_str.split("|").collect();
            let key = new_rule[0].parse::<i32>().ok().unwrap();
            let value = new_rule[1].parse::<i32>().ok().unwrap();

            rules_map.entry(key.clone()).or_insert_with(Vec::new).push(value);
        }
        else if in_str.contains(",") {
            let eval: Vec<&str> = in_str.split(",").collect();
            if process_report(&eval, &rules_map) {
                total = total + eval.get(eval.len() / 2).unwrap().parse::<i32>().ok().unwrap();
            }
            
        }
    }
    println!("{:?}", total);


}

fn find_smallest_index(find_in: &Vec<i32>, find_from: &Vec<i32>) -> Option<usize> {
    find_from.iter()
        .filter_map(|&val| {
            find_in.iter().position(|&x| x == val)
        })
        .min()
}

fn reorder(report: &Vec<&str>, rules_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut new_order: Vec<i32> = vec![];
    
    for item in report {
        let number = item.parse::<i32>().ok().unwrap();
        if rules_map.contains_key(&number) {
            match find_smallest_index(&new_order, &rules_map[&number]) {
                Some(index) => new_order.insert(index, number),
                None => new_order.push(number)
            }
        }
    }
    return new_order;
}

fn day_five_star_two() {
    let file = File::open("day5.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in reader.lines() {
        let in_str = line.ok().unwrap();
        if in_str.contains("|") {
            let new_rule: Vec<&str> = in_str.split("|").collect();
            let key = new_rule[0].parse::<i32>().ok().unwrap();
            let value = new_rule[1].parse::<i32>().ok().unwrap();

            rules_map.entry(key.clone()).or_insert_with(Vec::new).push(value);
        }
        else if in_str.contains(",") {
            let eval: Vec<&str> = in_str.split(",").collect();
            if !process_report(&eval, &rules_map) {
                let new_order = reorder(&eval, &rules_map);
                total = total + new_order.get(new_order.len() / 2).unwrap();
            }
            
        }
    }
    println!("{:?}", total);


}

fn day_six_star_one() {
    let file = File::open("day6.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let targets = ['^', '>', '<', 'V', 'v'];
    let mut current_position = None;
    let mut facing = None;
    let mut unique_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut x_delta: i32 = 0;
    let mut y_delta: i32 = 0;

    let lines: Vec<Vec<char>> = reader
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let line = line.unwrap();
            if current_position.is_none() {
                if let Some((col, character)) = line.chars().enumerate().find(|(_, c)| targets.contains(&c)) {
                    current_position = Some((row, col));
                    facing = Some(character);
                    unique_visited.insert((row, col));
                }
            }
            line.chars().collect()
        })
        .collect();
    if facing.unwrap() == '^' {
        x_delta = 0;
        y_delta = -1;
    } else if facing.unwrap() == '>' {
        x_delta = 1;
        y_delta = 0;
    } else if facing.unwrap() == '<' {
        x_delta = -1;
        y_delta = 0;
    } else {
        x_delta = 0;
        y_delta = 1;
    }

    loop {
        let (current_y, current_x) = current_position.unwrap();
        if (current_x as i32 + x_delta) < 0 || (current_x as i32 + x_delta) >= lines.get(0).unwrap().len() as i32 || (current_y as i32 + y_delta) < 0 || (current_y as i32 + y_delta) >= lines.len() as i32 {
            unique_visited.insert((current_y, current_x));
            break;
        }
        if let Some(row) = lines.get((current_y as i32 + y_delta) as usize) {
            if let Some(&cell) = row.get((current_x as i32 + x_delta) as usize) {
                if cell == '#' {
                    if x_delta == 1 {
                        x_delta = 0;
                        y_delta = 1;
                    } else if x_delta == -1 {
                        x_delta = 0;
                        y_delta = -1;
                    } else if y_delta == 1 {
                        x_delta = -1;
                        y_delta = 0;
                    } else {
                        x_delta = 1;
                        y_delta = 0;
                    }
                } else {
                    unique_visited.insert((current_y, current_x));
                    current_position = Some(((current_y as i32 + y_delta) as usize, (current_x as i32 + x_delta) as usize))
                }
            }
        }
    }
    // println!("{:?}", unique_visited);
    println!("{:?}", unique_visited.len());

}

fn test_obstruction_causes_loop(mut lines: Vec<Vec<char>>, starting_vector: (usize, usize, (i32, i32))) -> bool {
    let mut current_vector = starting_vector.clone();
    let mut unique_visited: HashSet<(usize, usize, (i32, i32))> = HashSet::new();
    unique_visited.insert(current_vector);
    
    loop {
        let (current_y, current_x, (y_delta, x_delta)) = current_vector;
        if (current_x as i32 + x_delta) < 0 || (current_x as i32 + x_delta) >= lines.get(0).unwrap().len() as i32 || (current_y as i32 + y_delta) < 0 || (current_y as i32 + y_delta) >= lines.len() as i32 {
            return false;
        }
        if let Some(row) = lines.get((current_y as i32 + y_delta) as usize) {
            if let Some(&cell) = row.get((current_x as i32 + x_delta) as usize) {
                if cell == '#' {
                    if x_delta == 1 {
                        current_vector = (current_y, current_x, (1, 0));
                    } else if x_delta == -1 {
                        current_vector = (current_y, current_x, (-1, 0));
                    } else if y_delta == 1 {
                        current_vector = (current_y, current_x, (0, -1));
                    } else {
                        current_vector = (current_y, current_x, (0, 1));
                    }
                } else {
                    // lines[current_y][current_x] = 'X';
                    current_vector = ((current_y as i32 + y_delta) as usize, (current_x as i32 + x_delta) as usize, (y_delta, x_delta));
                }
                if !unique_visited.insert(current_vector) {
                    // println!("{:?}", lines);
                    // let mut file = File::create("output.txt").unwrap();
                    // for line in lines {
                    //     writeln!(file, "{}", line.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")).ok();
                    // }
                    // println!("{:?}", unique_visited);
                    // println!("{:?}", current_vector);
                    return true;
                }
            }
        }
    }
}

fn day_six_star_two() {
    let file = File::open("day6.txt").ok().unwrap();
    let reader = BufReader::new(file);
    let targets = ['^', '>', '<', 'V', 'v'];
    let mut current_vector = None;
    let mut facing = None;
    let mut unique_visited: HashSet<(usize, usize, (i32, i32))> = HashSet::new();
    // let mut total_valid_obstructions = 0;
    let mut placed_obstructions: HashSet<(usize, usize)> = HashSet::new();
    let mut start_vector = None;

    let mut lines: Vec<Vec<char>> = reader
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let line = line.unwrap();
            if current_vector.is_none() {
                if let Some((col, character)) = line.chars().enumerate().find(|(_, c)| targets.contains(&c)) {
                    let mut x_delta: i32 = 0;
                    let mut y_delta: i32 = 0;
                    facing = Some(character);
                    if facing.unwrap() == '^' {
                        x_delta = 0;
                        y_delta = -1;
                    } else if facing.unwrap() == '>' {
                        x_delta = 1;
                        y_delta = 0;
                    } else if facing.unwrap() == '<' {
                        x_delta = -1;
                        y_delta = 0;
                    } else {
                        x_delta = 0;
                        y_delta = 1;
                    }
                    start_vector = Some((row, col, (y_delta, x_delta)));
                    current_vector = start_vector;

                    unique_visited.insert(current_vector.unwrap());
                }
            }
            line.chars().collect()
        })
        .collect();

    loop {
        let (current_y, current_x, (y_delta, x_delta)) = current_vector.unwrap();
        if (current_x as i32 + x_delta) < 0 || (current_x as i32 + x_delta) >= lines.get(0).unwrap().len() as i32 || (current_y as i32 + y_delta) < 0 || (current_y as i32 + y_delta) >= lines.len() as i32 {
            break;
        }
        if let Some(row) = lines.get((current_y as i32 + y_delta) as usize) {
            if let Some(&cell) = row.get((current_x as i32 + x_delta) as usize) {
                if cell == '#' {
                    if x_delta == 1 {
                        current_vector = Some((current_y, current_x, (1, 0)));
                    } else if x_delta == -1 {
                        current_vector = Some((current_y, current_x, (-1, 0)));
                    } else if y_delta == 1 {
                        current_vector = Some((current_y, current_x, (0, -1)));
                    } else {
                        current_vector = Some((current_y, current_x, (0, 1)));
                    }
                } else {
                    let mut lines_with_new_obstruction = lines.clone();
                    let next_position_y = (current_y as i32 + y_delta) as usize;
                    let next_position_x = (current_x as i32 + x_delta) as usize;

                    lines_with_new_obstruction[next_position_y][next_position_x] = '#';

                    if placed_obstructions.get(&(next_position_y, next_position_x)).is_none() && test_obstruction_causes_loop(lines_with_new_obstruction, start_vector.unwrap()) {
                        placed_obstructions.insert((next_position_y, next_position_x));
                    }
                    current_vector = Some((next_position_y, next_position_x, (y_delta, x_delta)))
                }
                unique_visited.insert(current_vector.unwrap());
            }
        }
    }
    println!("{:?}", placed_obstructions.len());

}

