#![no_std]
#![no_main]

mod uart;

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    uart::initialize();
    for b in "Coming back to where you started is not the same as never leaving.\n".bytes() {
        uart::putchar(b);
    }

    loop {}
}

