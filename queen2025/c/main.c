
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>

// 获取当前毫秒时间戳
int64_t clock_realtime() {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}
uint32_t solve(int row, uint32_t cols, uint32_t diag1, uint32_t diag2, int n, uint32_t all) {
    if (row == n) return 1;
    uint32_t count = 0;
    uint32_t available = all & ~(cols | diag1 | diag2);
    while (available) {
        uint32_t bit = available & -available;
        available ^= bit;
        count += solve(row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1, n, all);
    }
    return count;
}
// 递归+位运算八皇后
uint32_t count_n_queens(int n) {
    if (n == 0 || n > 32) return 0;
    uint32_t all = (1U << n) - 1;
    return solve(0, 0, 0, 0, n, all);
}

// 非递归+位运算八皇后
uint32_t count_n_queens_non_recursive(int n) {
    if (n == 0 || n > 32) return 0;
    uint32_t all = (1U << n) - 1;
    typedef struct { int row; uint32_t cols, diag1, diag2; } state_t;
    state_t *stack = (state_t*)malloc(sizeof(state_t) * n * n);
    int top = 0;
    stack[top++] = (state_t){0, 0, 0, 0};
    uint32_t count = 0;
    while (top > 0) {
        state_t s = stack[--top];
        if (s.row == n) {
            count++;
            continue;
        }
        uint32_t available = all & ~(s.cols | s.diag1 | s.diag2);
        while (available) {
            uint32_t bit = available & -available;
            available ^= bit;
            stack[top++] = (state_t){s.row + 1, s.cols | bit, (s.diag1 | bit) << 1, (s.diag2 | bit) >> 1};
        }
    }
    free(stack);
    return count;
}

int main(int argc, char *argv[]) {
    int n = 13;
    if (argc > 1) {
        int v = atoi(argv[1]);
        if (v > 0 && v <= 32) n = v;
    }
    int64_t t1 = clock_realtime();
    uint32_t count = count_n_queens(n);
    int64_t t2 = clock_realtime();
    printf("C [recursive] n=%d found=%u time=%ld ms\n", n, count, t2-t1);

    int64_t t3 = clock_realtime();
    uint32_t count2 = count_n_queens_non_recursive(n);
    int64_t t4 = clock_realtime();
    printf("C [non-recursive] n=%d found=%u time=%ld ms\n", n, count2, t4-t3);
    return 0;
}
