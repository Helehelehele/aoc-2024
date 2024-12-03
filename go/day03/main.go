package main

import (
	"aoc2024/utils"
	"regexp"
	"strconv"
	"strings"
)

func runCalculations(path string, ignoreEnabledCheck bool) int {
	input := utils.ReadInput(path)

	inputStr := strings.Join(input, "")

	re := regexp.MustCompile(`(do\(\)|don't\(\))|mul\((\d{1,3}),(\d{1,3})\)`)

	matches := re.FindAllStringSubmatch(inputStr, -1)

	enabled := true
	total := 0

	for _, match := range matches {
		switch first := match[0]; first {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if enabled || ignoreEnabledCheck {
				num1, _ := strconv.Atoi(match[2])
				num2, _ := strconv.Atoi(match[3])
				total += num1 * num2
			}
		}
	}

	return total
}

func Part1() {
	result := runCalculations("day03/input.txt", true)
	println("Part 1: ", result)
}

func Part2() {
	result := runCalculations("day03/input.txt", false)
	println("Part 2: ", result)
}

func main() {
	Part1()
	Part2()
}
