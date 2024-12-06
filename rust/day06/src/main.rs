use std::collections::HashSet;
use utils;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn next(&self, direction: char) -> Position {
        let delta = match direction {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => (0, 0),
        };
        Position {
            row: self.row + delta.0,
            col: self.col + delta.1,
        }
    }

    fn is_valid(&self, rows: i32, cols: i32) -> bool {
        self.row >= 0 && self.row < rows && self.col >= 0 && self.col < cols
    }
}

#[derive(Hash, Eq, PartialEq)]
struct State {
    pos: Position,
    direction: char,
}

const DIRECTIONS: [char; 4] = ['^', '>', 'v', '<'];
const NEXT_DIRECTION: [(char, char); 4] = [('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')];

fn get_next_direction(direction: char) -> char {
    NEXT_DIRECTION
        .iter()
        .find(|&&(curr, _)| curr == direction)
        .map_or(' ', |&(_, next)| next)
}

fn build_grid(path: &str) -> Vec<Vec<char>> {
    utils::read_input(path)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_guard(grid: &[Vec<char>]) -> (char, Position) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if DIRECTIONS.contains(&cell) {
                return (
                    cell,
                    Position {
                        row: i as i32,
                        col: j as i32,
                    },
                );
            }
        }
    }
    (' ', Position { row: 0, col: 0 })
}

fn simulate_movement(
    grid: &[Vec<char>],
    initial_direction: char,
    initial_pos: Position,
    visited: &mut HashSet<Position>,
) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut current_pos = initial_pos;
    let mut current_direction = initial_direction;

    loop {
        let next_pos = current_pos.next(current_direction);

        if !next_pos.is_valid(rows, cols) {
            break;
        }

        if grid[next_pos.row as usize][next_pos.col as usize] == '#' {
            current_direction = get_next_direction(current_direction);
            continue;
        }

        visited.insert(next_pos);
        current_pos = next_pos;
    }
}

fn creates_loop(
    grid: &[Vec<char>],
    initial_direction: char,
    start_pos: Position,
    obstacle_pos: Position,
) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visited_states = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_direction = initial_direction;

    let max_states = (rows * cols * 4) as usize; 

    loop {
        let next_pos = current_pos.next(current_direction);
        let state = State {
            pos: current_pos,
            direction: current_direction,
        };

        if !visited_states.insert(state) {
            return true;
        }

        if visited_states.len() > max_states {
            return false; 
        }

        if !next_pos.is_valid(rows, cols) {
            return false;
        }

        if next_pos == obstacle_pos || grid[next_pos.row as usize][next_pos.col as usize] == '#' {
            current_direction = get_next_direction(current_direction);
            continue;
        }

        current_pos = next_pos;
    }
}

fn part1() {
    let grid = build_grid("day06/input.txt");
    let (direction, pos) = find_guard(&grid);
    let mut visited = HashSet::new();
    visited.insert(pos);

    simulate_movement(&grid, direction, pos, &mut visited);
    println!("Part 1: {}", visited.len());
}

fn part2() {
    let grid = build_grid("day06/input.txt");
    let (initial_direction, initial_pos) = find_guard(&grid);
    let rows = grid.len();
    let cols = grid[0].len();

    let total = (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .filter(|&(row, col)| {
            let pos = Position {
                row: row as i32,
                col: col as i32,
            };
            pos != initial_pos
                && grid[row][col] != '#'
                && creates_loop(&grid, initial_direction, initial_pos, pos)
        })
        .count();

    println!("Part 2: {}", total);
}

fn main() {
    part1();
    part2();
}
