package main

import (
	"aoc2024/utils"
	"strconv"
	"strings"
)

func buildReports(path string) [][]int {
	input := utils.ReadInput(path)

	var reports [][]int

	for _, line := range input {
		numberStrings := strings.Fields(line)

		var levels []int

		for _, numberString := range numberStrings {
			number, _ := strconv.Atoi(numberString)
			levels = append(levels, number)
		}

		reports = append(reports, levels)
	}

	return reports
}

func isSafe(report []int) bool {
	if len(report) < 2 {
		return true
	}

	increasing := true
	decreasing := true

	for i := 1; i < len(report); i++ {
		delta := report[i] - report[i-1]

		if utils.Abs(delta) < 1 || utils.Abs(delta) > 3 {
			return false
		}

		if delta > 0 {
			decreasing = false
		} else if delta < 0 {
			increasing = false
		}
	}

	return increasing || decreasing
}

func createCombinations(report []int, size int) <-chan []int {
	c := make(chan []int)

	go func() {
		defer close(c)

		if size == 1 {
			for _, number := range report {
				c <- []int{number}
			}
		} else {
			for i := 0; i < len(report); i++ {
				for combination := range createCombinations(report[i+1:], size-1) {
					c <- append([]int{report[i]}, combination...)
				}
			}
		}
	}()

	return c
}

func Part1(reports [][]int) {
	count := 0

	for _, report := range reports {
		if isSafe(report) {
			count++
		}
	}

	println("Part 1: ", count)
}

func Part2(reports [][]int) {
	count := 0

	for _, report := range reports {
		if isSafe(report) {
			count++
		} else {
			for combination := range createCombinations(report, len(report)-1) {
				if isSafe(combination) {
					count++
					break
				}
			}
		}
	}

	println("Part 2: ", count)
}

func main() {
	reports := buildReports("day02/input.txt")
	Part1(reports)
	Part2(reports)
}
