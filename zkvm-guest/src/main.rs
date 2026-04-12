#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::{self, Write};

struct GuestWriter;

impl Write for GuestWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = s.as_ptr();
        let len = s.len();
        unsafe {
            core::arch::asm!(
                "ecall",
                in("a7") 1, // Syscall 1 (Print)
                in("a0") ptr,
                in("a1") len,
            );
        }
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*)@{ (let _ = core::fmt::write(&mut GuestWriter, format_args!($($arg)*))); });
}

macro_rules! println {
    () => (macro_rules_print!("\n"));
    ($($arg:tt)*) => ({
        print!($($arg)*);
        print!("\n");
    });
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world from the zkVM!");
    
    unsafe {
        core::arch::asm!("ebreak");
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loan {}
}
