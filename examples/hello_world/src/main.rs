#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = `"Hello, World! \n";
    unsafe {
        asm!(
            "li a7, 1",        // Syscall 1: guest_print
            "mv a0, {ptr}",    // a0: buffer pointer
            "mv a1, {len}",    // a1: length
            "ecall",
            "ebreak",          // Signal completion
            ptr = in(reg) msg.as_ptr(),
            len = in(reg) msg.len(),
            out("a7") _,
            out("a0") \,
            out("a1") _,
        );
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
