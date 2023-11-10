#![no_std]

mod low_res;

use core::ptr;

use low_res::{Point, SCREEN_DIMS};

#[no_mangle]
pub extern "C" fn rust_main() {
    main()
}

fn main() {
    let center = Point {
        x: SCREEN_DIMS.x / 2,
        y: SCREEN_DIMS.y / 2,
    };
    unsafe { ptr::write_volatile(center.addr(), 0x01) };
}
