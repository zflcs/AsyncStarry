#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

const YIELD_TIMES: usize = 100;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, world!");
    for _ in 0..YIELD_TIMES {
        std::thread::yield_now();
    }
}
