package main

import (
	"strconv"
	"strings"
)

type Dot struct {
	x, y int
}

type Paper struct {
	dots []Dot
}

func (p *Paper) add(dot Dot) {
	p.dots = append(p.dots, dot)
}

func (p Paper) distinct() (distinctDots map[Dot]bool) {
	distinctDots = make(map[Dot]bool)
	for _, dot := range p.dots {
		distinctDots[dot] = true
	}

	return
}

func (p *Paper) foldX(x int) {
	var foldedPaper []Dot
	for _, dot := range p.dots {
		if dot.x > x {
			dot.x -= 2 * (dot.x - x)
		}

		foldedPaper = append(foldedPaper, dot)
	}

	p.dots = foldedPaper
}

func (p *Paper) foldY(y int) {
	var foldedPaper []Dot
	for _, dot := range p.dots {
		if dot.y > y {
			dot.y -= 2 * (dot.y - y)
		}
		foldedPaper = append(foldedPaper, dot)
	}
	p.dots = foldedPaper
}

func makePaper(input []string) (paper Paper) {
	for _, line := range input {
		parts := strings.Split(line, ",")
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		paper.add(Dot{x: x, y: y})
	}

	return
}
