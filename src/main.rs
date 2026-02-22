// Tells the rust compiler that this code will run in
// an environment without an operating system
#![no_std]

// Tells the compiler that there's no main function and that
// we'll manually handle the entry point for the code
#![no_main]

// Link some default handler for hardware exceptions
use cortex_m_rt as _;
// This will do some low level initialization thats out of scope
// for this workshop
use embassy_rp::init;
// Struct defined by the rust `core` library that contains the
// message and source code location of a panic (exception)
use core::panic::PanicInfo;

// Rust will insert code to automatically call this function
// whenever an irrecoverable error occurs in the program. It
// can never return (hence it returns !) since panics are errors that are deadly to
// the execution of a program
#[panic_handler]
fn panic(_pi: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let _ = init(Default::default());

    loop {}
}
