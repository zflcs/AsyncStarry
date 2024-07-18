#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::println;
use std::thread;

#[cfg_attr(feature = "axstd", axstd::async_main)]
#[cfg_attr(feature = "axstd", no_mangle)]
async fn main() -> i32 {
    println!("Hello, world!");
    let a = thread::spawn(move || async {
        println!("Hello from a thread!");
        32
    }).join().await;
    println!("a = {}", a.unwrap());
    let a = thread::spawn(move || async {
        println!("Hello from a thread!");
        32
    }).join().await;
    println!("a = {}", a.unwrap());
    0
}
