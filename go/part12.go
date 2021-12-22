package main

import (
	"fmt"
	"strings"
)

type edge struct {
	from string
	to   string
}

func traverse(edges []edge, path []string) (paths [][]string) {
	current := path[len(path)-1]
	if current == "end" {
		fmt.Printf("Ok! path %+v \n", path)
		return [][]string{path}
	}

	for _, edge := range edges {
		if edge.from == current && (edge.to == "end" || edge.to == strings.ToUpper(edge.to) || isLowercaseAllowed(path, edge.to)) {
			paths = append(paths, traverse(edges, append(path, edge.to))...)
		}
	}

	return
}

func computePossiblePaths(input []string) int {
	edges := make([]edge, 0)
	for _, line := range input {
		parts := strings.Split(line, "-")
		if parts[0] == "start" || parts[1] == "end" {
			edges = append(edges, edge{from: parts[0], to: parts[1]})
		} else {
			edges = append(edges, edge{from: parts[0], to: parts[1]}, edge{from: parts[1], to: parts[0]})
		}
	}

	return len(traverse(edges, []string{"start"}))
}

func isLowercaseAllowed(path []string, test string) bool {
	var testCount int
	var otherOccursTwice = false
	lowercases := make(map[string]int)
	for _, entry := range path {
		if entry == strings.ToLower(entry) {
			lowercases[entry] = lowercases[entry] + 1
		}
		if entry == test {
			testCount++
		} else if lowercases[entry] >= 2 {
			otherOccursTwice = true
		}
	}

	result := (otherOccursTwice && testCount < 1) || (!otherOccursTwice && testCount <= 1)
	return result
}
