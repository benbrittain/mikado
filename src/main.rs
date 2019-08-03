#![no_std]
#![no_main]
#![feature(asm)]

mod clint;
mod memory_region;
mod riscv;
mod trap;
mod uart;

use clint::Clint;
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
    loop {
        riscv::wfi()
    }
}

pub fn main() -> Result<(), core::fmt::Error> {
    uart::initialize();
    println!("Coming back to where you started is not the same as never leaving.\n");

    let result = riscv::misa();
    println!("result: {:x}", result);

    unsafe { asm!("ebreak") }

    println!("Passed breakpoint.");

    write_csr!(0xC80, 0x01);

    println!("{:?}", riscv::mstatus());

    riscv::set_mie(true, true, true);
    riscv::enable_mie();

    println!("{:?}", riscv::mstatus());

    let mut clint = Clint::new();
    println!("time: {:?}", clint.set_time_cmp(200));
    println!("time: {:?}", clint.get_time_cmp());
    println!("time: {:?}", clint.get_time());
    println!("time: {:?}", clint.get_time());
    println!("time: {:?}", clint.get_time());

    Ok(())
}
