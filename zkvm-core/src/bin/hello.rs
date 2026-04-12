#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello from zkVM !\n";
    let ptr = msg.as_ptr();
    let len = msg.len();

    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") 1, // Syscall 1 (Print)
            in("a0") ptr,
            in("a1") len,
        );
        core::arch::asm!("ebreak");
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
