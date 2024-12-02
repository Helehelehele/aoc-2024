use utils;

fn build_reports(path: &str) -> Vec<Vec<i32>> {
    let input = utils::read_input(path);
    
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|x| x.parse::<i32>())
            .map(|x| x.unwrap())
            .collect::<Vec<i32>>()
        )
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {
        let delta = report[i] - report[i - 1];

        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }

        if delta < 0 {
            increasing = false;
        } else if delta > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing

}

fn part1(reports: Vec<Vec<i32>>) {
    let mut safe_count = 0;

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        }
    }

    println!("Part 1: {}", safe_count);
}

fn part2(reports: Vec<Vec<i32>>) {
    let mut safe_count = 0;

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        } else {
            let size = report.len();
            for i in 0..size {
                let mut new_report = report.clone();
                new_report.remove(i);

                if is_safe(&new_report) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2: {}", safe_count);
}

fn main() {
    let reports = build_reports("day02/input.txt");

    part1(reports.clone());
    part2(reports);
}
