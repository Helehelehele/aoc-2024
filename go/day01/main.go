package main

import (
	"aoc2024/utils"
	"slices"
	"strconv"
	"strings"
)

func buildLists(path string) ([]int, []int) {
	input := utils.ReadInput(path)

	var list1 []int
	var list2 []int

	for _, line := range input {
		numberStrings := strings.Fields(line)
		number1, err := strconv.Atoi(numberStrings[0])

		if err != nil {
			panic(err)
		}

		number2, err := strconv.Atoi(numberStrings[1])

		if err != nil {
			panic(err)
		}

		list1 = append(list1, number1)
		list2 = append(list2, number2)
	}

	return list1, list2
}

func Part1(list1 []int, list2 []int) {
	slices.Sort(list1)
	slices.Sort(list2)

	difference := 0

	for i := 0; i < len(list1); i++ {
		difference += utils.Abs(list1[i] - list2[i])
	}

	println("Part 1: ", difference)
}

func Part2(list1 []int, list2 []int) {
	total := 0

	occurances := make(map[int]int)
	minIndex := 0

	for i := 0; i < len(list1); i++ {
		current := list1[i]

		if count, ok := occurances[current]; ok {
			total += count * current
		} else {
			occurances[current] = 0
			for j := minIndex; j < len(list2); j++ {
				if list2[j] == current {
					occurances[current]++
				}
				if list2[j] > current {
					minIndex = j
					break
				}
			}
			total += occurances[current] * current
		}

	}

	println("Part 2: ", total)
}

func main() {
	list1, list2 := buildLists("day01/input.txt")
	Part1(list1, list2)
	Part2(list1, list2)
}
