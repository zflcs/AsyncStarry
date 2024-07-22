#![cfg_attr(not(test), no_std)]
#![no_main]

#[axstd::async_main]
async fn main() -> i32 {
    axstarry::fs_init();
    let testcase = "busybox sh";
    axstarry::run_testcase(testcase);
    axstarry::println(format!("System halted with exit code {}", 0).as_str());
    0
}