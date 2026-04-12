#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::{self, Write};

pub struct GuestWriter;

impl Write for GuestWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = s.as_ptr();
        let len = s.len();
        #[cfg(target_arch = "riscv32")]
        unsafe {
            core::arch::asm!(
                "ecall",
                in("a7") 1,
                in("a0") ptr,
                in("a1") len,
            );
        }
        #[cfg(not(target_arch = "riscv32"))]
        {
           let _ = (ptr, len);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! guest_print {
    ($($arg:tt)*) => {
        let _ = core::fmt::write(&mut $crate::GuestWriter, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! guest_println {
    () => ($crate::guest_print!("\n"));
    ($($arg:tt)*) => ({
        $crate::guest_print!($($arg))*;
        $crate::guest_print!("\n");
    });
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    guest_println!("Hello, world from the zkVM!");

    #[cfg(target_arch = "riscv32")]
    unsafe {
        core::arch::asm!("ebreak");
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        #[cfg(target_arch = "riscv32")]
        unsafe {
            core::arch::asm!("ebreak");
        }
    }
}
