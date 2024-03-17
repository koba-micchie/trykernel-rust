
use core::arch::asm;
use crate::sysdef::*;

// input from 32bit register
pub fn in_w(addr: u32) -> u32 {
    unsafe {
        *(addr as *const u32) 
    }
}

// output to 32bit register
pub fn out_w(addr: u32, data: u32) {
    unsafe {
        *(addr as *mut u32) = data
    }
}

// output to 32bit register (bit clear) 
const OP_CLR: u32 = 0x3000;
pub fn clr_w(addr: u32, data: u32) {
    out_w(addr + OP_CLR, data);
}

// output to 32bit register (bit set) 
const OP_SET: u32 = 0x2000;
pub fn set_w(addr: u32, data: u32) {
    out_w(addr + OP_SET, data);
}

// output to 32bit register (bit EXOR) 
const OP_XOR: u32 = 0x1000;
pub fn xset_w(addr: u32, data: u32) {
    out_w(addr + OP_XOR, data);
}

// inline function to control PRIMASK register 
pub fn set_primask(pm: isize) {
    unsafe {
        asm!("msr primask, {}", in(reg) pm);
    }
}

pub fn get_primask() -> isize {
    let mut pm: isize;
    unsafe {
        asm!("mrs {}, primask", out(reg) pm);
    }
    pm 
}

// disable interrupt
pub fn di() -> isize {
    let intsts:isize = get_primask();
    set_primask(1);
    intsts
}

// enable interrupt
pub fn ei(intsts: isize) {
    set_primask(intsts);
}

// UART0 initialize
pub fn tm_com_init() {
    // set baud-rate 
    out_w(UART0_BASE + UARTX_IBRD, 67);
    out_w(UART0_BASE + UARTX_FBRD, 52);
    // set data format
    out_w(UART0_BASE + UARTX_LCR_H, 0x70);
    // enable interface
    out_w(UART0_BASE + UARTX_CR, UART_CR_RXE | UART_CR_TXE | UART_CR_EN);
}

pub fn tm_putstring(str: &str) -> usize {

    let mut cnt: usize = 0;
    for c in str.chars() { 
        // wait for send FIFO
        while in_w(UART0_BASE + UARTX_FR) & UART_FR_TXFF != 0 {}
        // send data
        let data = c as u32;
        out_w(UART0_BASE + UARTX_DR, data);
        cnt += 1;
    }
    cnt
}


