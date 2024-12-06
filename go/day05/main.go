package main

import (
	"aoc2024/utils"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func parseRulesAndUpdates(path string) (map[int][]int, [][]int) {
	lines := utils.ReadInput(path)

	rules := map[int][]int{}
	updates := [][]int{}

	i := 0

	for i < len(lines) {
		line := lines[i]
		if !strings.Contains(line, "|") {
			i++ // Skip the empty line
			break
		}

		pageStrings := strings.SplitN(line, "|", 2)
		firstPage, _ := strconv.Atoi(pageStrings[0])
		secondPage, _ := strconv.Atoi(pageStrings[1])

		rules[firstPage] = append(rules[firstPage], secondPage)

		i++
	}

	for i < len(lines) {
		line := lines[i]
		if !strings.Contains(line, ",") {
			break
		}

		pageStrings := strings.Split(line, ",")
		pages := []int{}

		for _, pageString := range pageStrings {
			page, _ := strconv.Atoi(pageString)
			pages = append(pages, page)
		}

		updates = append(updates, pages)
		i++
	}

	return rules, updates
}

func isCorrect(rules map[int][]int, update []int) bool {
	for before, after_list := range rules {
		for _, after := range after_list {
			if !checkOrder(update, before, after) {
				return false
			}
		}
	}

	return true
}

func checkOrder(update []int, before int, after int) bool {
	if slices.Contains(update, before) && slices.Contains(update, after) {
		return slices.Index(update, before) < slices.Index(update, after)
	}

	return true
}

func getMiddleElement(update []int) int {
	if len(update) == 0 {
		return -1
	}

	if len(update)%2 == 0 {
		return update[len(update)/2-1]
	}

	return update[len(update)/2]
}

func reorder(rules map[int][]int, update []int) {
	slices.SortFunc(update, func(i, j int) int {
		if values, exists := rules[i]; exists && slices.Contains(values, j) {
			return -1
		}
		if values, exists := rules[j]; exists && slices.Contains(values, i) {
			return 1
		}
		return 0
	})
}

func Part1() {
	rules, updates := parseRulesAndUpdates("day05/input.txt")

	total := 0

	for _, update := range updates {
		if isCorrect(rules, update) {
			total += getMiddleElement(update)
		}
	}

	fmt.Println("Part 1", total)
}

func Part2() {
	rules, updates := parseRulesAndUpdates("day05/input.txt")

	total := 0

	for _, update := range updates {
		if !isCorrect(rules, update) {
			reorder(rules, update)
			total += getMiddleElement(update)
		}
	}

	fmt.Println("Part 2", total)
}

func main() {
	Part1()
	Part2()
}
