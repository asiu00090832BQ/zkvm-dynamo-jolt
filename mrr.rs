#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello, from root!
";
    
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") 1,
            in("a0") msg.as_ptr(),
            in("a1") msg.len(),
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} }