use crate::typedef::*;

// Execution context information on stack 
pub struct StackFrame {
    // REG R4-R11 
    _r_: [u32; 8],
    // REG R0-R3
    _r:  [u32; 4],
    // REG R12
    _ip: u32,
    // REG lr
    _lr: u32,
    // REG pc
    pc: u32,
    // REG xpsr
    xpsr: u32,
}

// Create initial execution context 
pub fn make_context(sp: u32, ssize: u32, fp: FP) -> *mut StackFrame  {
    // Set sfp to pointer to execution context information on stack
    let sfp: *mut StackFrame = (sp + ssize - core::mem::size_of::<StackFrame>() as u32) as *mut StackFrame;

    // Initialize execution context information
    unsafe {
        (*sfp).xpsr = 0x0100_0000;
        (*sfp).pc   = fp & !0x0000_0001;
    }

    return sfp;
}
