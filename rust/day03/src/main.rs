use regex::Regex;
use utils;

fn solve(path: &str, ignore_enable: bool) -> i32 {
    let input = utils::read_input(path);
    
    let re = Regex::new(r"(do\(\)|don't\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
 
    let result = re.captures_iter(&input);

    let mut enabled = true;
    let mut total = 0;
  
    for mat in result {
        match mat.get(1) {
            Some(m) if m.as_str() == "do()" => {
                enabled = true;
            }
            Some(m) if m.as_str() == "don't()" => {
                enabled = false;
            }
            Some(_) => { 
                panic!("Invalid match");
            }
            None => {
                if enabled || ignore_enable {
                    let a = mat.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let b = mat.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    total += a * b;
                }
            }
        }
    }

    return total;
}


fn part1(path: &str) {
    let result = solve(path, true);

    println!("Part 1: {}", result);
}

fn part2(path: &str) {
    let result = solve(path, false);

    println!("Part 2: {}", result);
}

fn main() {
    part1("day03/input.txt");
    part2("day03/input.txt");
}
