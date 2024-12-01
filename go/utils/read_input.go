package utils

import (
	"bufio"
	"os"
)

func ReadInput(path string) []string {
	file, err := os.Open("../input/" + path)

	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)

	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines
}
