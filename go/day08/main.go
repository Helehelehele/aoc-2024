package main

import (
	"aoc2024/utils"
	"fmt"
	"math"
)

type Point struct {
	row, col int
}

func (p Point) GetDelta(q Point) Point {
	return Point{q.row - p.row, q.col - p.col}
}

type Grid [][]string
type AntennaPositions map[string][]Point

func parseInput(path string) Grid {
	lines := utils.ReadInput(path)
	grid := make(Grid, len(lines))

	for i, line := range lines {
		grid[i] = make([]string, len(line))
		for j, ch := range line {
			grid[i][j] = string(ch)
		}
	}

	return grid
}

func findAntennas(grid Grid) AntennaPositions {
	antennas := make(AntennaPositions)

	for i := range grid {
		for j, cell := range grid[i] {
			if cell != "." {
				antennas[cell] = append(antennas[cell], Point{i, j})
			}
		}
	}

	return antennas
}

func isInBounds(grid Grid, p Point) bool {
	return p.row >= 0 && p.row < len(grid) && p.col >= 0 && p.col < len(grid[0])
}

func generateCombinations(positions []Point, size int) [][]Point {
	if size == 0 {
		return [][]Point{{}}
	}
	if len(positions) == 0 {
		return nil
	}

	result := [][]Point{}
	head := positions[0]
	tail := positions[1:]

	for _, combo := range generateCombinations(tail, size-1) {
		result = append(result, append([]Point{head}, combo...))
	}
	result = append(result, generateCombinations(tail, size)...)

	return result
}

func calculateMaxSteps(rows, cols, dx, dy int) int {
	maxDim := max(rows, cols)
	maxDelta := max(utils.Abs(dx), utils.Abs(dy))
	if maxDelta == 0 {
		return 1
	}
	return int(math.Ceil(float64(maxDim) / float64(maxDelta)))
}

func findAntinodes(grid Grid, p1, p2 Point, limitDistance bool) []Point {
	delta := p1.GetDelta(p2)
	result := []Point{}

	if limitDistance {
		candidates := []Point{
			{p1.row - delta.row, p1.col - delta.col},
			{p2.row + delta.row, p2.col + delta.col},
		}
		for _, p := range candidates {
			if isInBounds(grid, p) {
				result = append(result, p)
			}
		}
	} else {
		maxSteps := calculateMaxSteps(len(grid), len(grid[0]), delta.row, delta.col)
		for i := -maxSteps; i <= maxSteps; i++ {
			candidates := []Point{
				{p1.row - i*delta.row, p1.col - i*delta.col},
				{p2.row + i*delta.row, p2.col + i*delta.col},
			}
			for _, p := range candidates {
				if isInBounds(grid, p) {
					result = append(result, p)
				}
			}
		}
	}

	return result
}

func findAllAntinodes(grid Grid, limitDistance bool) map[Point]bool {
	antennas := findAntennas(grid)
	uniquePoints := make(map[Point]bool)

	for _, positions := range antennas {
		if len(positions) <= 1 {
			continue
		}

		for _, combo := range generateCombinations(positions, 2) {
			antinodes := findAntinodes(grid, combo[0], combo[1], limitDistance)
			for _, p := range antinodes {
				uniquePoints[p] = true
			}
		}
	}

	return uniquePoints
}

func Part1() {
	grid := parseInput("day08/input.txt")
	result := findAllAntinodes(grid, true)
	fmt.Println("Part 1:", len(result))
}

func Part2() {
	grid := parseInput("day08/input.txt")
	result := findAllAntinodes(grid, false)
	fmt.Println("Part 2:", len(result))
}

func main() {
	Part1()
	Part2()
}
