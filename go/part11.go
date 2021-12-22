package main

import (
	"strconv"
	"strings"
)

type position struct {
	x int
	y int
}

func modelSteps(numSteps int, input []string) (steps int) {
	octos := make([][]int, 10)
	for i := 0; i < 10; i++ {
		octos[i] = make([]int, 10)
	}

	for y, line := range input {
		for x, digit := range strings.Split(line, "") {
			octo, _ := strconv.Atoi(digit)
			octos[y][x] = octo
		}
	}

	for {
		steps++
		if computeFlashes(octos) == 100 {
			break
		}
	}

	return
}

func computeFlashes(octos [][]int) int {
	for y := range octos {
		for x := range octos[y] {
			octos[y][x]++
		}
	}

	var toFlash []position
	for {

		flashed := false
		for y := range octos {
			for x := range octos[y] {
				center := position{x, y}
				octo := octos[y][x]
				if octo > 9 && !hasFlashed(toFlash, center) {
					flashed = true
					toFlash = append(toFlash, center)
					for _, adjacent := range neighbors(center) {
						if !hasFlashed(toFlash, adjacent) && adjacent.x >= 0 && adjacent.x < 10 && adjacent.y >= 0 && adjacent.y < 10 {
							octos[adjacent.y][adjacent.x]++
						}
					}
				}
			}
		}

		if !flashed {
			break
		}
	}

	for _, flashed := range toFlash {
		octos[flashed.y][flashed.x] = 0

	}

	return len(toFlash)
}

func neighbors(center position) []position {
	x := center.x
	y := center.y
	return []position{{x, y - 1},
		{x + 1, y - 1},
		{x + 1, y},
		{x + 1, y + 1},
		{x, y + 1},
		{x - 1, y + 1},
		{x - 1, y},
		{x - 1, y - 1}}
}

func hasFlashed(flashed []position, test position) bool {
	for _, flash := range flashed {
		if test == flash {
			return true
		}
	}

	return false
}
