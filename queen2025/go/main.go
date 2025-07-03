package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func clockRealtime() int64 {
	return time.Now().UnixNano() / 1e6
}

// 高性能八皇后解法计数，递归+位运算
func countNQueens(n int) uint32 {
	var solve func(row int, cols, diag1, diag2, all uint32) uint32
	solve = func(row int, cols, diag1, diag2, all uint32) uint32 {
		if row == n {
			return 1
		}
		count := uint32(0)
		available := all & ^(cols | diag1 | diag2)
		for available != 0 {
			bit := available & -available
			available ^= bit
			count += solve(row+1, cols|bit, (diag1|bit)<<1, (diag2|bit)>>1, all)
		}
		return count
	}
	if n == 0 || n > 32 {
		return 0
	}
	all := uint32((1 << n) - 1)
	return solve(0, 0, 0, 0, all)
}

// 非递归位运算八皇后
func countNQueensNonRecursive(n int) uint32 {
	if n == 0 || n > 32 {
		return 0
	}
	all := uint32((1 << n) - 1)
	type state struct {
		row   int
		cols  uint32
		diag1 uint32
		diag2 uint32
	}
	stack := make([]state, 0, n)
	stack = append(stack, state{0, 0, 0, 0})
	count := uint32(0)
	for len(stack) > 0 {
		s := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if s.row == n {
			count++
			continue
		}
		available := all & ^(s.cols | s.diag1 | s.diag2)
		for available != 0 {
			bit := available & -available
			available ^= bit
			stack = append(stack, state{s.row + 1, s.cols | bit, (s.diag1 | bit) << 1, (s.diag2 | bit) >> 1})
		}
	}
	return count
}

func main() {
	n := 13
	if len(os.Args) > 1 {
		if v, err := strconv.Atoi(os.Args[1]); err == nil && v > 0 && v <= 32 {
			n = v
		}
	}
	t1 := clockRealtime()
	count := countNQueens(n)
	t2 := clockRealtime()
	fmt.Printf("Go [recursive] n=%d found=%d time=%d ms\n", n, count, t2-t1)

	t3 := clockRealtime()
	count2 := countNQueensNonRecursive(n)
	t4 := clockRealtime()
	fmt.Printf("Go [non-recursive] n=%d found=%d time=%d ms\n", n, count2, t4-t3)
}
