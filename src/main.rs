#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln, debug};
extern crate alloc;
use alloc::string::String;
use alloc_cortex_m::CortexMHeap;
use stm32f4::stm32f407;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    let message = String::from("Hello there!");
    hprintln!("{}", message).unwrap();

    // initializing the uart
    

    debug::exit(debug::EXIT_SUCCESS);

    let mut peripherals = stm32f407::Peripherals::take().unwrap();
    let uart = &peripherals.USART1;

    loop{
        
    }
}