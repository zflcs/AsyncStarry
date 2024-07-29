#![no_std]
#![no_main]

#[macro_use]
extern crate axstd as std;


#[axstd::async_main]
#[no_mangle]
async fn main() {
    const YIELD_TIMES: usize = 1000_0000;
    let mut a = 0;
    while a < YIELD_TIMES {
        std::thread::yield_now();
        // println!("Yield test: {}", a);
        a += 1;
    }
    println!("Yield tests run OK!");
    0
}
