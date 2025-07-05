#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        unsafe {
            Hall_Clear(0);
        }
    }
}
