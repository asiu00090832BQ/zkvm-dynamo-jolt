#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello World!\n";
    print(msg);
    loop {}
}

fn print(s: &str) {
    let ptr = s.as_ptr() as u32;
    let len = s.len() as u32;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") 1,
            in("a0") ptr,
            in("a1") len,
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
