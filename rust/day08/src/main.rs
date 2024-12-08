use std::collections::{HashMap, HashSet};
use utils;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn get_delta(&self, other: &Point) -> Point {
        Point {
            row: other.row - self.row,
            col: other.col - self.col,
        }
    }
}

type Grid = Vec<Vec<char>>;
type AntennaPositions = HashMap<char, Vec<Point>>;

fn build_grid(path: &str) -> Grid {
    utils::read_input(path)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_antennas(grid: &Grid) -> AntennaPositions {
    let mut antennas = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas
                    .entry(cell)
                    .or_insert_with(Vec::new)
                    .push(Point {
                        row: i as i32,
                        col: j as i32,
                    });
            }
        }
    }

    antennas
}

fn is_in_bounds(grid: &Grid, point: &Point) -> bool {
    point.row >= 0
        && point.row < grid.len() as i32
        && point.col >= 0
        && point.col < grid[0].len() as i32
}

fn generate_combinations(positions: &[Point], size: usize) -> Vec<Vec<Point>> {
    if size == 0 {
        return vec![vec![]];
    }
    if positions.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    let head = positions[0];
    let tail = &positions[1..];

    for mut combo in generate_combinations(tail, size - 1) {
        combo.insert(0, head);
        result.push(combo);
    }
    result.extend(generate_combinations(tail, size));

    result
}

fn calculate_max_steps(rows: i32, cols: i32, dx: i32, dy: i32) -> i32 {
    let max_dim = rows.max(cols);
    let max_delta = dx.abs().max(dy.abs());
    if max_delta == 0 {
        return 1;
    }
    ((max_dim as f64) / (max_delta as f64)).ceil() as i32
}

fn find_antinodes(grid: &Grid, p1: &Point, p2: &Point, limit_distance: bool) -> Vec<Point> {
    let delta = p1.get_delta(p2);
    let mut result = Vec::new();

    if limit_distance {
        let candidates = vec![
            Point {
                row: p1.row - delta.row,
                col: p1.col - delta.col,
            },
            Point {
                row: p2.row + delta.row,
                col: p2.col + delta.col,
            },
        ];
        for p in candidates {
            if is_in_bounds(grid, &p) {
                result.push(p);
            }
        }
    } else {
        let max_steps = calculate_max_steps(
            grid.len() as i32,
            grid[0].len() as i32,
            delta.row,
            delta.col,
        );
        for i in -max_steps..=max_steps {
            let candidates = vec![
                Point {
                    row: p1.row - i * delta.row,
                    col: p1.col - i * delta.col,
                },
                Point {
                    row: p2.row + i * delta.row,
                    col: p2.col + i * delta.col,
                },
            ];
            for p in candidates {
                if is_in_bounds(grid, &p) {
                    result.push(p);
                }
            }
        }
    }

    result
}

fn find_all_antinodes(grid: &Grid, limit_distance: bool) -> HashSet<Point> {
    let antennas = find_antennas(grid);
    let mut unique_points = HashSet::new();

    for positions in antennas.values() {
        if positions.len() <= 1 {
            continue;
        }

        for combo in generate_combinations(positions, 2) {
            let antinodes = find_antinodes(grid, &combo[0], &combo[1], limit_distance);
            for p in antinodes {
                unique_points.insert(p);
            }
        }
    }

    unique_points
}

fn part1(grid: &Grid) {
    let result = find_all_antinodes(grid, true);
    println!("Part 1: {}", result.len());
}

fn part2(grid: &Grid) {
    let result = find_all_antinodes(grid, false);
    println!("Part 2: {}", result.len());
}

fn main() {
    let grid = build_grid("day08/input.txt");
    part1(&grid);
    part2(&grid);
}
