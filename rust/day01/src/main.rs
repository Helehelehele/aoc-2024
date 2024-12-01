use std::collections::HashMap;

use utils;

fn build_lists(path: &str) -> (Vec<i32>, Vec<i32>) {
    let input = utils::read_input(path);
    
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|x| x.parse::<i32>())
            .map(|x| x.unwrap())
            .collect::<Vec<i32>>()
        )
        .fold((Vec::new(), Vec::new()), |(mut list1, mut list2), x| {
            if x.len() == 2 {
                list1.push(x[0]);
                list2.push(x[1]);
            }
            (list1, list2)
        })
}

fn part1(list1: Vec<i32>, list2: Vec<i32>) {
    let mut difference: i32 = 0;

    for i in 0..list1.len() {
        difference += (list1[i] - list2[i]).abs();
    }

    println!("Part 1: {}", difference);
}

fn part2(list1: Vec<i32>, list2: Vec<i32>) {
    let mut total = 0;

    let mut occurances: HashMap<i32, i32> = HashMap::new();
    let mut min_index: usize = 0;

    for i in 0..list1.len() {
        let current = list1[i];

        if let Some(count) = occurances.get(&current) {
            total += current * count;
        } else {
            occurances.insert(current, 0);
            for j in min_index..list2.len() {
                if list2[j] == current {
                    let count = occurances.get_mut(&current).unwrap();
                    *count += 1;
                }
                if list2[j] > current {
                    min_index = j;
                    break;
                }
            }
            let count = occurances.get(&current).unwrap();
            total += current * count;
        }
    }

    println!("Part 2: {}", total);
}

fn main() {
    let (mut list1, mut list2) = build_lists("day01/input.txt");
    list1.sort();
    list2.sort();
    part1(list1.clone(), list2.clone());
    part2(list1, list2);
}
