use std::collections::HashMap;

use utils;

fn parse_rules_and_updates(path: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let input = utils::read_input(path);

    let lines: Vec<&str> = input.lines().collect();

    let rules = lines.iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split("|");
            let before = parts.next().unwrap();
            let after = parts.next().unwrap();

            let before_num = before.parse::<i32>().unwrap();
            let after_num = after.parse::<i32>().unwrap();

            (before_num, after_num)
        })
        .fold(HashMap::<i32, Vec<i32>>::new(),
            |acc, (before, after)| {
                // insert after into before's list
                let mut new_acc = acc.clone();
                let list = new_acc.entry(before).or_insert(Vec::new());
                list.push(after);
                new_acc
        });

    let updates = lines.iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let nums = line.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            nums
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, updates)
}

fn check_order(update: &Vec<i32>, before : i32, after: i32) -> bool {
    if update.contains(&before) && update.contains(&after) {
        let before_index = update.iter().position(|&x| x == before).unwrap();
        let after_index = update.iter().position(|&x| x == after).unwrap();

        return before_index < after_index;
    }

    true
}

fn is_correct(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    rules.iter().all(|(before, after_list)| {
        after_list.iter().all(|after| {
            check_order(update, *before, *after)
        })
    })
}

fn get_middle_item(update: &Vec<i32>) -> i32 {
    if update.len() == 0 {
        return -1;
    }

    if update.len() % 2 == 0 {
        return update[update.len() / 2];
    }

    return update[(update.len() - 1) / 2];
}

fn reorder(rules: &HashMap<i32, Vec<i32>>, update: &mut Vec<i32>) {
    update.sort_by(|a, b| {
        if rules.get(a).map_or(false, |values| values.contains(b)) {
            return std::cmp::Ordering::Less;
        } 
        if rules.get(b).map_or(false, |values| values.contains(a)) {
            return std::cmp::Ordering::Greater;
        }

        return std::cmp::Ordering::Equal;
    });
}

fn part1(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut total = 0;

    for update in updates {
        if is_correct(rules, update) {
            total += get_middle_item(update);
        }
    }

    println!("Part 1: {}", total);
}

fn part2(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut total = 0;

    for update in updates {
        if !is_correct(rules, update) {
            let mut reordered = update.clone();
            reorder(rules, &mut reordered);
            total += get_middle_item(&reordered);
        }
    }

    println!("Part 2: {}", total);
}

fn main() {
    let (rules, updates) = parse_rules_and_updates("day05/input.txt");

    part1(&rules, &updates);
    part2(&rules, &updates);
}

