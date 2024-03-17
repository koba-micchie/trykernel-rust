
use crate::syslib::*;
use crate::error::*;

// debug print
pub fn debug( str: &str ) {

    tm_putstring("DEBUG: ");
    tm_putstring( str );

}

// error print
pub fn error( err: KernelError ) {

    let msg: &'static str;

    match err {
      KernelError::OK    => { msg = "No error"; }
      KernelError::SYS   => { msg = "System Error"; }
      KernelError::NOCOP => { msg = "No Coprocessor"; }
      KernelError::NOSPT => { msg = "No Support Function"; }
      KernelError::RSFN  => { msg = "Reserved Function"; }
      KernelError::RSATR => { msg = "Reserved Attribute"; }
      KernelError::PAR   => { msg = "Parameter Invalid"; }
      KernelError::ID    => { msg = "ID Invalid"; }
      KernelError::CTX   => { msg = "Context Error"; }
      KernelError::MACV  => { msg = "Memory Access Violation"; }
      KernelError::OACV  => { msg = "Object Access Violation"; }
      KernelError::ILUSE => { msg = "Illigal Use system call"; }
      KernelError::NOMEM => { msg = "No Memory"; }
      KernelError::LIMIT => { msg = "Hi Limit Exceeded"; }
      KernelError::OBJ   => { msg = "Object Status Error"; }
      KernelError::NOEXS => { msg = "Object Invalid Error"; }
      KernelError::QOVR  => { msg = "Queue Limit Exceeded"; }
      KernelError::RLWAI => { msg = "Release WAIT Status"; }
      KernelError::TMOUT => { msg = "Time Out"; }
      KernelError::DLT   => { msg = "Delete Wait Obejct"; }
      KernelError::DISWAI => { msg = "Release Wait Status by Prohibit Wait"; }
      KernelError::IO    => { msg = "I/O Error"; } 
    }
 
    tm_putstring("Kernel ERROR: ");
    tm_putstring( msg );

}
