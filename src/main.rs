#![no_std]
#![no_main]

mod uart;

use core::panic::PanicInfo;

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
    println!("Coming back to where you started is not the same as never leaving.");

    Ok(())
}
