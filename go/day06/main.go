package main

import (
	"aoc2024/utils"
	"fmt"
)

type Position struct {
	row, col int
}

type State struct {
	pos       Position
	direction string
}

var directions = map[string]Position{
	"^": {-1, 0},
	">": {0, 1},
	"v": {1, 0},
	"<": {0, -1},
}

func getNextDirection(direction string) string {
	switch direction {
	case "^":
		return ">"
	case ">":
		return "v"
	case "v":
		return "<"
	case "<":
		return "^"
	default:
		return ""
	}
}

func getNextPosition(pos Position, direction string) Position {
	delta := directions[direction]
	return Position{pos.row + delta.row, pos.col + delta.col}
}

func buildGrid(path string) [][]string {
	lines := utils.ReadInput(path)
	grid := make([][]string, len(lines))
	for i, line := range lines {
		grid[i] = make([]string, len(line))
		for j, ch := range line {
			grid[i][j] = string(ch)
		}
	}
	return grid
}

func findGuard(grid [][]string) (string, Position) {
	for i := range grid {
		for j := range grid[i] {
			cell := grid[i][j]
			if _, ok := directions[cell]; ok {
				return cell, Position{i, j}
			}
		}
	}
	return "", Position{}
}

func isValidPosition(grid [][]string, pos Position) bool {
	return pos.row >= 0 && pos.row < len(grid) && pos.col >= 0 && pos.col < len(grid[0])
}

func isObstacle(grid [][]string, pos Position) bool {
	return grid[pos.row][pos.col] == "#"
}

func simulateMovement(grid [][]string, direction string, pos Position, visited map[Position]bool) map[Position]bool {
	nextPos := getNextPosition(pos, direction)

	if !isValidPosition(grid, nextPos) {
		return visited
	}

	if isObstacle(grid, nextPos) {
		newDirection := getNextDirection(direction)
		return simulateMovement(grid, newDirection, pos, visited)
	}

	visited[nextPos] = true
	return simulateMovement(grid, direction, nextPos, visited)
}

func createsLoop(grid [][]string, direction string, startPos Position, obstaclePos Position, visitedStates map[State]bool) bool {
	nextPos := getNextPosition(startPos, direction)
	state := State{startPos, direction}

	if visitedStates[state] {
		return true
	}

	if !isValidPosition(grid, nextPos) {
		return false
	}

	visitedStates[state] = true

	if nextPos == obstaclePos || isObstacle(grid, nextPos) {
		newDirection := getNextDirection(direction)
		return createsLoop(grid, newDirection, startPos, obstaclePos, visitedStates)
	}

	return createsLoop(grid, direction, nextPos, obstaclePos, visitedStates)
}

func Part1() {
	grid := buildGrid("day06/input.txt")
	direction, pos := findGuard(grid)
	visited := make(map[Position]bool)
	visited[pos] = true

	result := simulateMovement(grid, direction, pos, visited)
	fmt.Println("Part 1", len(result))
}

func Part2() {
	grid := buildGrid("day06/input.txt")
	initialDirection, initialPos := findGuard(grid)

	total := 0
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[0]); col++ {
			pos := Position{row, col}
			if pos != initialPos && !isObstacle(grid, pos) {
				visitedStates := make(map[State]bool)
				if createsLoop(grid, initialDirection, initialPos, pos, visitedStates) {
					total++
				}
			}
		}
	}

	fmt.Println("Part 2", total)
}

func main() {
	Part1()
	Part2()
}
