
use crate::typedef::*;

// Timeout 0
pub const TMO_POL: i32 = 0;
// Infinit wait 
pub const TMO_FEVR: i32 = -1;

//タスク生成情報
pub struct TCtsk {
    // Task attribute 
    pub tskatr: ATR,
    // Task program address 
    pub task: FP,
    // Task priority
    pub itskpri: PRI,
    // Stack size of Task 
    pub stksz: SZ,
    // Buffer pointer of Stack
    pub bufptr: u32,
}

// Task Attribute 
pub const TA_HLNG:    u32 = 0x0000_0001;
pub const TA_USERBUF: u32 = 0x0000_0020;
pub const TA_RNG0:    u32 = 0x0000_0000;
pub const TA_RNG1:    u32 = 0x0000_0100;
pub const TA_RNG2:    u32 = 0x0000_0200;
pub const TA_RNG3:    u32 = 0x0000_0300;

// Manage wait task by FIFO
pub const TA_TFIFO: usize = 0x0000_0000;
// Manage wait task by priority
pub const TA_TPRI:  usize = 0x0000_0001;
// Manage wait task by top of queue 
pub const TA_FIRST: usize = 0x0000_0000;
// Manage wait task by least number of requests
pub const TA_CNT:   usize = 0x0000_0002;
// Do not allow multiple wait tasks 
pub const TA_WFGL:  usize = 0x0000_0000;
// Allow multiple wait tasks 
pub const TA_WMUL:  usize = 0x0000_0008;

// Information of Event flag creation 
pub struct TCflg  {
    // Attribute of Event flag 
    pub flgatr: ATR,
    // Initial value of Event flag
    pub iflgptn: usize
}

// Wait AND
pub const TWF_ANDW:   usize = 0x0000_0000;
// Wait OR
pub const TWF_ORW:    usize = 0x0000_0001;
// All bit clear 
pub const TWF_CLR:    usize = 0x0000_0010;
// Only condition bit clear 
pub const TWF_BITCLR: usize = 0x0000_0020;

pub struct TCsem {
    // Attribute of semaphore 
    pub sematr: ATR,
    // Initial of Semaphore resource number 
    pub isemcnt: isize,
    // Max value of Semaphore resource number
    pub maxsem: isize,
}


