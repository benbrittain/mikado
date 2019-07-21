#![no_std]
#![no_main]
#![feature(asm)]

mod trap;
mod uart;

use core::panic::PanicInfo;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        crate::uart::Writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    // Anything returned from main that isn't Ok is a system-wide panic
    if let Err(error) = main() {
        panic!("{}", error);
    }
    loop {}
}

pub fn main() -> Result<(), core::fmt::Error> {
    uart::initialize();
    println!("Coming back to where you started is not the same as never leaving.\n");

    let result: u32;
    unsafe { asm!("csrr $0, $1" : "=r"(result) : "i"(0x301)) }
    println!("result: {:x}", result);

    unsafe { asm!("ebreak") }

    println!("Passed breakpoint.");

    Ok(())
}
