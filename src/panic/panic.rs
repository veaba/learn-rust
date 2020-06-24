// panic!与不可恢复的错误 https://woodsking2.gitbooks.io/rust/content/ch09-01-unrecoverable-errors-with-panic.html



fn main() {
    panic!("❤❤❤❤❤❤❤❤❤")
}

// 最后三行 说明是造成的错误信息

/*
F:\Github\learn-rust>cargo run
   Compiling learn-rust v0.1.0 (F:\Github\learn-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.77s
     Running `target\debug\learn-rust.exe`
thread 'main' panicked at 'Clear up!', src\main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\learn-rust.exe` (exit code: 101)
*/