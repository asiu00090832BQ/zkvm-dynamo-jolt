#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

#[cfg(target_arch = "riscv32")]
use core::panic::PanicInfo;

#[cfg(target_arch = "riscv32")]
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

#[cfg(all(target_arch = "riscv32", not(test)))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_arch = "riscv32"))]
fn main() {}