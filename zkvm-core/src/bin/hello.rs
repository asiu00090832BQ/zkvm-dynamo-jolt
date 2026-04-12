#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
static mut RESULT: u32 = 0;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut sum: u32 = 0;
    for i in 0..=100 {
        sum = sum.wrapping_add(i);
    }
    unsafe {
        RESULT = sum;
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
