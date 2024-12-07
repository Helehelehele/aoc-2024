use utils;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

struct Equation {
    test_value: i64,  
    numbers: Vec<i64>, 
}

fn parse_input(path: &str) -> Vec<Equation> {
    let input = utils::read_input(path);
    
    input.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let test_value = parts[0].trim().parse::<i64>().unwrap();  
            let numbers = parts[1].trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap()) 
                .collect();
            
            Equation { test_value, numbers }
        })
        .collect()
}

fn generate_operator_combinations(count: usize, allow_concat: bool) -> Vec<Vec<Operator>> {
    let available_ops = if allow_concat {
        vec![Operator::Add, Operator::Multiply, Operator::Concat]
    } else {
        vec![Operator::Add, Operator::Multiply]
    };

    let mut result = vec![vec![]];
    
    for _ in 0..count {
        let mut new_combinations = Vec::new();
        for combination in result {
            for &op in &available_ops {
                let mut new_comb = combination.clone();
                new_comb.push(op);
                new_combinations.push(new_comb);
            }
        }
        result = new_combinations;
    }
    
    result
}

fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 { 
    let mut result = numbers[0];

    for (i, &op) in operators.iter().enumerate() {
        match op {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concat => {
                result = format!("{}{}", result, numbers[i + 1])
                    .parse()
                    .unwrap();
            }
        }
    }

    result
}

fn is_equation_valid(eq: &Equation, allow_concat: bool) -> bool {
    let operator_count = eq.numbers.len() - 1;
    let combinations = generate_operator_combinations(operator_count, allow_concat);

    combinations.iter().any(|operators| {
        evaluate(&eq.numbers, operators) == eq.test_value
    })
}

fn part1(equations: &[Equation]) {
    let total: i64 = equations 
        .iter()
        .filter(|eq| is_equation_valid(eq, false))
        .map(|eq| eq.test_value)
        .sum();

    println!("Part 1: {}", total);
}

fn part2(equations: &[Equation]) {
    let total: i64 = equations
        .iter()
        .filter(|eq| is_equation_valid(eq, true))
        .map(|eq| eq.test_value)
        .sum();

    println!("Part 2: {}", total);
}

fn main() {
    let equations = parse_input("day07/input.txt");
    part1(&equations);
    part2(&equations);
}
