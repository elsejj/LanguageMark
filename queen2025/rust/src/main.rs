use std::time::{SystemTime, UNIX_EPOCH};

fn clock_realtime() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_millis() as i64;
}

// 高性能八皇后解法计数，使用位运算和递归，符合 Rust 最佳实践
fn count_n_queens(n: usize) -> u32 {
    fn solve(row: usize, cols: u32, diag1: u32, diag2: u32, n: usize, all: u32) -> u32 {
        if row == n {
            return 1;
        }
        let mut count = 0;
        let mut available = all & !(cols | diag1 | diag2);
        while available != 0 {
            let bit = available & (!available + 1); // 取最低位的1
            available ^= bit;
            count += solve(
                row + 1,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
                n,
                all,
            );
        }
        count
    }
    if n == 0 || n > 32 {
        return 0;
    }
    let all = (1u32 << n) - 1;
    solve(0, 0, 0, 0, n, all)
}

// 非递归位运算八皇后
fn count_n_queens_non_recursive(n: usize) -> u32 {
    if n == 0 || n > 32 {
        return 0;
    }
    let all = (1u32 << n) - 1;
    let mut stack = Vec::with_capacity(n);
    let row = 0;
    let cols = 0u32;
    let diag1 = 0u32;
    let diag2 = 0u32;
    let mut count = 0u32;
    stack.push((row, cols, diag1, diag2));
    while let Some((row, cols, diag1, diag2)) = stack.pop() {
        if row == n {
            count += 1;
            continue;
        }
        let mut available = all & !(cols | diag1 | diag2);
        while available != 0 {
            let bit = available & (!available + 1);
            available ^= bit;
            stack.push((row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1));
        }
    }
    count
}

fn main() {
    use std::env;
    let mut n = 13;
    if let Some(arg1) = env::args().nth(1) {
        if let Ok(v) = arg1.parse::<usize>() {
            if v > 0 && v <= 32 {
                n = v;
            }
        }
    }
    let t1 = clock_realtime();
    let count = count_n_queens(n);
    let t2 = clock_realtime();
    println!(
        "Rust [recursive] n={} found={} time={} ms",
        n,
        count,
        t2 - t1
    );
    // 非递归解法
    let t3 = clock_realtime();
    let count2 = count_n_queens_non_recursive(n);
    let t4 = clock_realtime();
    println!(
        "Rust [non-recursive] n={} found={} time={} ms",
        n,
        count2,
        t4 - t3
    );
}
