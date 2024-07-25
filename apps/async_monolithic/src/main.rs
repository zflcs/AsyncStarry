#![cfg_attr(not(test), no_std)]
#![no_main]

#[macro_use]
extern crate axstd;

#[axstd::async_main]
#[no_mangle]
async fn main() -> i32 {
    axstarry::fs_init();
    let testcase = "busybox sh";
    println!("Running testcase: {}", testcase);
    axstarry::run_testcase(testcase);
    axstarry::println(format!("System halted with exit code {}", 0).as_str());
    0
}