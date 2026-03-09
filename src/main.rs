#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate panic_halt;

// const RUST_ICON: &[u8] = include_bytes!("rust.raw");

use riscv_rt::entry;

fn rgb5551(r: u16, g: u16, b: u16, a: u16) -> u16 {
    ((a & 0x1) << 15) | ((b & 0x1F) << 10) | ((g & 0x1F) << 5) | (r & 0x1F)
}

#[entry]
fn main() -> ! {
    let mut counter = 0;
    loop {
        unsafe {
            let red = (libm::sin(counter.into()) * 0.5 + 0.5) * 31.0;
            let color = rgb5551(red as u16, 31 - red as u16, 15, 1);
            Hall_Clear(color);
            // Hall_Draw1(RUST_ICON.as_ptr() as *const u16, 0, 0, 0, 0, 480, 480, 480);
            while Hall_GetIsGPUBusy() {}
            let old_vsync = Hall_GetVSync();
            while Hall_GetVSync() == old_vsync {}
            Hall_SetCommandSwapBuffers();
        }
        counter += 1;
    }
}
