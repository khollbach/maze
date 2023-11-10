#![no_std]

mod low_res;

use core::ptr;

use low_res::{Point, SCREEN_DIMS};

#[no_mangle]
pub extern "C" fn rust_main() {
    main()
}

fn main() -> ! {
    ascii_table();
    loop {}
}

fn ascii_table() {
    for i in 0..16 {
        for j in 0..16 {
            let c = i << 4 | j;
            let p = Point {
                y: i as i8,
                x: j as i8,
            };
            unsafe { ptr::write_volatile(p.addr(), c) };
        }
    }
}

fn hello() {
    let s = "Hello, world!";
    debug_assert!(s.is_ascii());
    for (x, c) in s.chars().enumerate() {
        let y = SCREEN_DIMS.y / 2;
        unsafe { ptr::write_volatile(Point { x: x as i8, y }.addr(), c as u8) };
    }

    for i in 0..4 {
        for (x, c) in s.chars().enumerate() {
            let c = c as u8 ^ i << 6;
            let y = SCREEN_DIMS.y / 2 + i as i8 * 2;
            unsafe { ptr::write_volatile(Point { x: x as i8, y }.addr(), c) };
        }
    }
}
