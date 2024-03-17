
use core::panic::PanicInfo;

use crate::systimer::*;

// Define entry of vector table
pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

// Default handler
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler() {
    loop {}
}

// Reset handler (in asm.S)
extern "C" { fn Reset() -> !; }

// Dispatch handler for pend_sv
extern "C" { fn dispatch_entry(); }

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define Reset vector area in linker script
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
#[link_section = ".vector_table.exceptions"]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: DefaultHandler },  // nmi
    Vector { handler: DefaultHandler },  // hard_fault
    Vector { reserved: 0 },               // reserved0 
    Vector { reserved: 0 },               // reserved0 
    Vector { reserved: 0 },               // reserved0 
    Vector { reserved: 0 },               // reserved0 
    Vector { reserved: 0 },               // reserved0 
    Vector { reserved: 0 },               // reserved0 
    Vector { reserved: 0 },               // reserved0 
    Vector { handler: DefaultHandler },  // svcall
    Vector { reserved: 0 },               // reserved1
    Vector { reserved: 0 },               // reserved1
    Vector { handler: dispatch_entry },   // pend_sv
    Vector { handler: systimer_handler }, // sys_tick
];

#[no_mangle]
#[link_section = ".vector_table.interrupts"]
pub static INTERRUPS: [Vector; 32] = [
    Vector { handler: DefaultHandler },  // irq 0
    Vector { handler: DefaultHandler },  // irq 1
    Vector { handler: DefaultHandler },  // irq 2
    Vector { handler: DefaultHandler },  // irq 3
    Vector { handler: DefaultHandler },  // irq 4
    Vector { handler: DefaultHandler },  // irq 5
    Vector { handler: DefaultHandler },  // irq 6
    Vector { handler: DefaultHandler },  // irq 7
    Vector { handler: DefaultHandler },  // irq 8
    Vector { handler: DefaultHandler },  // irq 9
    Vector { handler: DefaultHandler },  // irq 10
    Vector { handler: DefaultHandler },  // irq 11
    Vector { handler: DefaultHandler },  // irq 12
    Vector { handler: DefaultHandler },  // irq 13
    Vector { handler: DefaultHandler },  // irq 14
    Vector { handler: DefaultHandler },  // irq 15
    Vector { handler: DefaultHandler },  // irq 16
    Vector { handler: DefaultHandler },  // irq 17
    Vector { handler: DefaultHandler },  // irq 18
    Vector { handler: DefaultHandler },  // irq 19
    Vector { handler: DefaultHandler },  // irq 20
    Vector { handler: DefaultHandler },  // irq 21
    Vector { handler: DefaultHandler },  // irq 22
    Vector { handler: DefaultHandler },  // irq 23
    Vector { handler: DefaultHandler },  // irq 24
    Vector { handler: DefaultHandler },  // irq 25
    Vector { handler: DefaultHandler },  // irq 26
    Vector { handler: DefaultHandler },  // irq 27
    Vector { handler: DefaultHandler },  // irq 28
    Vector { handler: DefaultHandler },  // irq 29
    Vector { handler: DefaultHandler },  // irq 30
    Vector { handler: DefaultHandler },  // irq 31
];
