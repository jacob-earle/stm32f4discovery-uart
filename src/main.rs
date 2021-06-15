#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};
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

    
    let peripherals = stm32f407::Peripherals::take().unwrap();
    let uart = &peripherals.USART3;

    // initializing gpio
    let gpiob = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;


    // initialize gpio clock
    rcc.ahb1enr.write(|w| w.gpioben().bit(true));
    // PB10 is our TX pin and PB11 is our RX pin, so initialize pin 10 as output and pin 11 as input
    gpiob.moder.modify(|_, w| w.moder11().bits(0b01).moder10().bits(0b00));
    // configure both pins' output speed to high
    gpiob.ospeedr.modify(|_, w| w.ospeedr11().bits(0b10).ospeedr10().bits(0b10));


    // initializing the uart for TX
    // 1. Enable the USART by writing the UE bit in USART_CR1 register to 1.
    uart.cr1.modify(|_, w|  w.ue().bit(true));

    // 2. Program the M bit in USART_CR1 to define the word length.
    // Setting to 0 transmits 8 bit words
    uart.cr1.modify(|_, w| w.m().bit(false));

    // 3. Program the number of stop bits in USART_CR2.
    // This is 0 by default, i.e. 1 stop bit, which is fine for our purposes
    uart.cr2.modify(|_,w| w.stop().bits(0));

    // 4. Select DMA enable (DMAT) in USART_CR3 if Multi buffer Communication is to take place. Configure the DMA register as explained in multibuffer communication.
    // For now, we will just attempt to send a single character, so we will not bother with DMA
    uart.cr3.modify(|_,w| w.dmat().bit(false));

    // 5. Select the desired baud rate using the USART_BRR register.
    // We will aim for a baudrate of 9600
    // The cpu has a default clock speed of 16 MHz, so the BRR value should be approx. 1666.67, which has integral part 1666 and fractional part approx. 11/16
    uart.brr.modify(|_,w| w.div_mantissa().bits(1666).div_fraction().bits(11));

    // 6. Set the TE bit in USART_CR1 to send an idle frame as first transmission.
    uart.cr1.modify(|_,w| w.te().bit(true));

    // 7. Write the data to send in the USART_DR register (this clears the TXE bit). Repeat this for each data to be transmitted in case of single buffer
    // We will write the character "F" to the uart, represented by ASCII code "0x46", five times
    for _ in 0..5{
        uart.dr.write(|w| w.dr().bits(0x46));
    }

    //debug::exit(debug::EXIT_SUCCESS);


    loop{
        
    }
}