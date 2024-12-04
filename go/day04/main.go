package main

import (
	"aoc2024/utils"
)

var (
	DiagonalUpRight   = Point{1, -1}
	DiagonalDownRight = Point{1, 1}
	Right             = Point{1, 0}
	Down              = Point{0, 1}
)

type Point struct {
	x, y int
}

type Grid [][]rune

func (p Point) Add(other Point) Point {
	return Point{p.x + other.x, p.y + other.y}
}

func (p Point) Scale(factor int) Point {
	return Point{p.x * factor, p.y * factor}
}

func parseGrid(path string) Grid {
	input := utils.ReadInput(path)

	grid := make(Grid, len(input))

	for i, line := range input {
		grid[i] = []rune(line)
	}

	return grid
}

func isInBounds(grid Grid, point Point) bool {
	return point.y >= 0 &&
		point.y < len(grid) &&
		point.x >= 0 &&
		point.x < len(grid[point.y])
}

func checkWord(grid Grid, word string, start Point, delta Point) bool {
	for i, letter := range word {
		current := start.Add(delta.Scale(i))
		if !isInBounds(grid, current) || grid[current.y][current.x] != letter {
			goto checkBackward
		}
	}
	return true

checkBackward:
	for i, letter := range utils.Reverse(word) {
		current := start.Add(delta.Scale(i))
		if !isInBounds(grid, current) || grid[current.y][current.x] != letter {
			return false
		}
	}
	return true
}

func countMatches(grid Grid, word string, start Point, xShapeOnly bool) int {
	if xShapeOnly {
		wordLen := len(word)
		end := start.Add(DiagonalDownRight.Scale(wordLen - 1))
		other := Point{start.x, end.y}

		if isInBounds(grid, end) &&
			checkWord(grid, word, start, DiagonalDownRight) &&
			checkWord(grid, word, other, DiagonalUpRight) {
			return 1
		}
		return 0
	}

	directions := []Point{DiagonalDownRight, DiagonalUpRight, Right, Down}
	total := 0
	for _, dir := range directions {
		if checkWord(grid, word, start, dir) {
			total++
		}
	}
	return total
}

func solve(path string, word string, xShapeOnly bool) int {
	grid := parseGrid(path)

	total := 0

	for y, row := range grid {
		for x := range row {
			total += countMatches(grid, word, Point{x, y}, xShapeOnly)
		}
	}

	return total
}

func Part1() {
	result := solve("day04/input.txt", "XMAS", false)
	println("Part 1:", result)
}

func Part2() {
	result := solve("day04/input.txt", "MAS", true)
	println("Part 2:", result)
}

func main() {
	Part1()
	Part2()
}
