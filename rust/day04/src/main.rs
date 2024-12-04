use utils;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: -1 },
];

fn build_grid(path: &str) -> Vec<Vec<char>> {
    let input = utils::read_input(path);

    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_in_bounds(grid: &Vec<Vec<char>>, point: Point) -> bool {
    point.y >= 0 
    && point.y < grid.len() as i32 
    && point.x >= 0 
    && point.x < grid[0].len() as i32
}

fn check_pattern(
    grid: &Vec<Vec<char>>, 
    pattern: impl Iterator<Item = char>,
    start: &Point, 
    direction: &Point,
) -> bool {
    for (i, expected_char) in pattern.enumerate() {
        let x = (start.x + direction.x * i as i32) as usize;
        let y = (start.y + direction.y * i as i32) as usize;
        
        if !is_in_bounds(grid, Point { x: x as i32, y: y as i32 }) 
            || grid[y][x] != expected_char {
            return false;
        }
    }
    true
}

fn check_word(
    grid: &Vec<Vec<char>>, 
    word: &str, 
    point: &Point, 
    direction: &Point
) -> bool {
    check_pattern(grid,  word.chars(),point, direction) // Forwards
        || check_pattern(grid, word.chars().rev(), point, direction) // Backwards
}

fn count_matches(
    grid: &Vec<Vec<char>>, 
    word: &str, 
    point: Point, 
    x_shape_only: bool
) -> i32 {
    if x_shape_only {
        let word_len = word.len() as i32;
        let end_point = Point { x: point.x + word_len - 1, y: point.y + word_len - 1 };
        let other = Point { x: point.x, y: end_point.y };

        return if is_in_bounds(grid, end_point)
            && check_word(grid, word, &point, &Point { x:1, y: 1 })
            && check_word(grid, word, &other, &Point { x:1, y: -1 }) {
            1
        } else {
            0
        };
    }

    DIRECTIONS.iter()
        .filter(|direction| check_word(grid, word, &point, direction))
        .count() as i32
}

fn solve(grid: &Vec<Vec<char>>, word: &str, x_shape_only: bool) -> i32 {
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            total += count_matches(
                grid, 
                word, 
                Point { x: x as i32, y: y as i32 },
                x_shape_only
            );
        }
    }

    total
}

fn part1(grid: &Vec<Vec<char>>) {
    let result = solve(grid, "XMAS", false);
    println!("Part 1: {}", result);
}

fn part2(grid: &Vec<Vec<char>>) {
    let result = solve(grid, "MAS", true);
    println!("Part 2: {}", result);
}

fn main() {
    let grid = build_grid("day04/input.txt");

    part1(&grid);
    part2(&grid);
}
