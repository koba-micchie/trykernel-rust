
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KernelError {
    OK    =  0, // No error 
    SYS   = -5, // System error 
    NOCOP = -6, // Co-processor unavailable 
    NOSPT = -9, // Unsupported function
    RSFN  =-10, // Reserved function number
    RSATR =-11, // Invalid attribute
    PAR   =-17, // Invalid parameter
    ID    =-18, // Invalid ID
    CTX   =-25, // Context error 
    MACV  =-26, // Memory access violation 
    OACV  =-27, // Invalid object access
    ILUSE =-28, // Invalid system call
    NOMEM =-33, // Memory shortage
    LIMIT =-34, // Limit reached
    OBJ   =-41, // Invalid object status
    NOEXS =-42, // Invalid object error
    QOVR  =-43, // Overflow
    RLWAI =-49, // Force release wait state 
    TMOUT =-50, // Time Out
    DLT   =-51, // Delete wait object
    DISWAI=-52, // Force release wait state by wait prohibit
    IO    =-57, // I/O error
}
