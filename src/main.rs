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

// Base address of the registers for controlling pin function
const IO_BANK0_BASE: usize = 0x40028000;
const PAD_CONTROL_BASE: usize = 0x40038000;
const SIO_BASE: usize = 0xd0000000;
const SIO_GPIO_OUT_SET: *mut u32 = (SIO_BASE + 0x018) as *mut u32;
const SIO_GPIO_OUT_CLEAR: *mut u32 = (SIO_BASE + 0x020) as *mut u32;
const SIO_GPIO_OUT_EN_SET: *mut u32 = (SIO_BASE + 0x038) as *mut u32;

const PIN_CONTROL_REGISTER_RESET: u32 = 0x0000001f;

fn pad_ctrl_addr(n: u8) -> *mut u32 {
    // + 4 since first register is VOLTAGE_SELECT
    (PAD_CONTROL_BASE + 4 + usize::from(n * 4)) as *mut u32
}

/// Returns the address of the status register for pin `n`
fn pin_status_addr(n: u8) -> *mut u32 {
    (IO_BANK0_BASE + usize::from(n * 8)) as *mut u32
}

/// Returns the address of the control register for pin `n`
fn pin_control_addr(n: u8) -> *mut u32 {
    unsafe { pin_status_addr(n).add(1) }
}

const PAD_CTRL_ISO_MASK: u32 = 1 << 8;

fn pad_ctrl_remove_iso(n: u8) {
    let pad_ctrl_reg = pad_ctrl_addr(n);
    unsafe {
        pad_ctrl_reg.write_volatile(*pad_ctrl_reg & !PAD_CTRL_ISO_MASK);
    }
}

const PAD_CTRL_IN_EN_SHIFT: u8 = 6;
const PAD_CTRL_IN_EN_MASK: u32 = 1 << PAD_CTRL_IN_EN_SHIFT;

fn pad_ctrl_input_enable(n: u8) {
    let pad_ctrl_reg = pad_ctrl_addr(n);
    unsafe {
        pad_ctrl_reg
            .write_volatile((*pad_ctrl_reg & !PAD_CTRL_IN_EN_MASK) | (1 << PAD_CTRL_IN_EN_SHIFT));
    }
}

fn reset_pin(n: u8) {
    // Writing to an arbitrary address is an unsafe operation
    //
    // Using an unsafe block here indicates to the compiler that we know what we're doing
    // and allows us to perform the operation
    unsafe {
        // We must do a volatile write to the address to indicate to the compiler that
        // this write has side-effects (changing pin function) that it cannot see.
        //
        // If we used a regular write, it would probably be optimized out since it seems useless to the
        // compiler.
        pin_control_addr(n).write_volatile(PIN_CONTROL_REGISTER_RESET);
    }
}

const PIN_FN_SEL_MASK: u32 = 0x1f;
const PIN_FUNCTION_USER_IO: u32 = 0x05;

fn set_pin_function(n: u8) {
    pad_ctrl_remove_iso(n);
    pad_ctrl_input_enable(n);
    let control_reg = pin_control_addr(n);
    unsafe {
        control_reg.write_volatile(
            (*control_reg & !PIN_FN_SEL_MASK) | PIN_FUNCTION_USER_IO,
        );
    }
}

// Only works for pin 0-31
fn pin_enable_output(n: u8) {
    unsafe { SIO_GPIO_OUT_EN_SET.write_volatile(1 << n) }
}

// Only works for pin 0-31
fn set_pin_high(n: u8) {
    unsafe { SIO_GPIO_OUT_SET.write_volatile(1 << n) }
}

fn set_pin_low(n: u8) {
    unsafe { SIO_GPIO_OUT_CLEAR.write_volatile(1 << n) }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let _ = init(Default::default());
    // reset_pin(25);
    set_pin_function(25);
    pin_enable_output(25);
    
    loop {
        set_pin_high(25);
        delay_cycles(20_000_000);
        set_pin_low(25);
        delay_cycles(20_000_000);
    }
}

/// Delays for approximately `n` iterations.
/// On the RP2350, each iteration takes roughly 3-4 clock cycles.
#[inline(always)]
fn delay_cycles(mut n: u32) {
    if n == 0 { return; }
    unsafe {
        core::arch::asm!(
            "2:",           // Label for the loop start
            "nop",          // Burn one cycle
            "subs {0}, #1", // Subtract 1 from the register (sets flags)
            "bne 2b",       // Branch back to '2' if not zero
            inout(reg) n => _,
            options(nomem, nostack),
        );
    }
}
