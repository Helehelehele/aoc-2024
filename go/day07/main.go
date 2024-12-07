package main

import (
	"aoc2024/utils"
	"fmt"
	"strconv"
	"strings"
)

type Operator string

const (
	Add      Operator = "add"
	Multiply Operator = "multiply"
	Concat   Operator = "concat"
)

var operators = []Operator{Add, Multiply, Concat}

type Equation struct {
	testValue int
	numbers   []int
}

func parseInput(path string) []Equation {
	lines := utils.ReadInput(path)
	equations := make([]Equation, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, ":")
		testValue, _ := strconv.Atoi(strings.TrimSpace(parts[0]))

		numberStrs := strings.Fields(strings.TrimSpace(parts[1]))
		numbers := make([]int, len(numberStrs))
		for j, numStr := range numberStrs {
			numbers[j], _ = strconv.Atoi(numStr)
		}

		equations[i] = Equation{testValue, numbers}
	}

	return equations
}

func generateOperatorCombinations(count int, allowConcat bool) [][]Operator {
	var availableOps []Operator
	if allowConcat {
		availableOps = operators
	} else {
		availableOps = []Operator{Add, Multiply}
	}

	result := [][]Operator{{}}
	for i := 0; i < count; i++ {
		var newCombinations [][]Operator
		for _, combination := range result {
			for _, op := range availableOps {
				newComb := make([]Operator, len(combination))
				copy(newComb, combination)
				newComb = append(newComb, op)
				newCombinations = append(newCombinations, newComb)
			}
		}
		result = newCombinations
	}
	return result
}

func evaluate(numbers []int, operators []Operator) int {
	result := numbers[0]

	for i := 0; i < len(operators); i++ {
		switch operators[i] {
		case Add:
			result += numbers[i+1]
		case Multiply:
			result *= numbers[i+1]
		case Concat:
			result, _ = strconv.Atoi(fmt.Sprintf("%d%d", result, numbers[i+1]))
		}
	}

	return result
}

func isEquationValid(eq Equation, allowConcat bool) bool {
	operatorCount := len(eq.numbers) - 1
	combinations := generateOperatorCombinations(operatorCount, allowConcat)

	for _, operators := range combinations {
		if evaluate(eq.numbers, operators) == eq.testValue {
			return true
		}
	}
	return false
}

func Part1() {
	equations := parseInput("day07/input.txt")
	total := 0

	for _, eq := range equations {
		if isEquationValid(eq, false) {
			total += eq.testValue
		}
	}

	fmt.Println("Part 1:", total)
}

func Part2() {
	equations := parseInput("day07/input.txt")
	total := 0

	for _, eq := range equations {
		if isEquationValid(eq, true) {
			total += eq.testValue
		}
	}

	fmt.Println("Part 2:", total)
}

func main() {
	Part1()
	Part2()
}
