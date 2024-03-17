use defer_lite::defer;

use core::result::Result;
use core::result::Result::{Ok,Err};

use crate::apidef::*;
use crate::config::*;
use crate::context::*;
use crate::knldef::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::typedef::*;
use crate::error::*;

// Task Control Block (TCB)
pub static mut TCB_TBL: [ TCB; CNF_MAX_TSK_ID ] = 
  [ 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
];

// Task ready queue for each priority
pub static mut READY_QUEUE: [ TcbQueue; CNF_MAX_TSK_PRI ] =
  [ TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
];

// Task wait queue 
pub static mut WAIT_QUEUE: TcbQueue = TcbQueue { head: core::ptr::null_mut() } ;

// Task in execute
#[no_mangle]
pub static mut  cur_task: *mut TCB = core::ptr::null_mut(); 
// Task scheduled 
#[no_mangle]
pub static mut sche_task: *mut TCB = core::ptr::null_mut();

// dispatch_entry() is in execute
#[no_mangle]
pub static mut disp_running: bool  = false;

// Task create API
pub fn tk_cre_tsk(pk_ctsk: &mut TCtsk) -> Result<ID, KernelError> {
    // Check arguments
    if ((*pk_ctsk).tskatr & !TA_RNG3) != (TA_HLNG | TA_USERBUF) {
        return Err(KernelError::RSATR);
    }
    if ((*pk_ctsk).itskpri <= 0) || ((*pk_ctsk).itskpri > CNF_MAX_TSK_PRI) {
        return Err(KernelError::PAR);
    }
    if (*pk_ctsk).stksz == 0 {
        return Err(KernelError::PAR);
    }

    let intsts = di();
    defer! {  ei(intsts); }
    
    // Search unused TCB
    let mut i: usize = 0;
    while i < CNF_MAX_TSK_ID {
        unsafe {
            if TCB_TBL[i].state == TSTAT::TsNonexist {
                break;
            }
        }
        i = i + 1;
    }
    // Task reached at limit
    if i >= CNF_MAX_TSK_ID {
        return Err(KernelError::LIMIT);
    }
    // Initialize TCB
    unsafe {
        TCB_TBL[i].state = TSTAT::TsDormant;
        TCB_TBL[i].pre   = core::ptr::null_mut();
        TCB_TBL[i].next  = core::ptr::null_mut();

        TCB_TBL[i].tskadr  = (*pk_ctsk).task;
        TCB_TBL[i].itskpri = (*pk_ctsk).itskpri;
        TCB_TBL[i].stksz   = (*pk_ctsk).stksz;
        TCB_TBL[i].stkadr =  (*pk_ctsk).bufptr;
    }
    Ok(i + 1)
}


// Task Start API
pub fn tk_sta_tsk(tskid: ID) -> Result<(), KernelError> {

    // Check arguments
    if (tskid <= 0) || (tskid > CNF_MAX_TSK_ID) {
         return Err(KernelError::ID);
    }

    let intsts: isize = di();
    defer! { ei(intsts); }

    unsafe {
        let tcb: *mut TCB = core::ptr::addr_of_mut!(TCB_TBL[tskid - 1]);
        if (*tcb).state != TSTAT::TsDormant {
            return Err(KernelError::OBJ);
        }
        // Change state of TCB ready to execute
        (*tcb).state = TSTAT::TsReady;
        (*tcb).context = make_context((*tcb).stkadr, (*tcb).stksz, (*tcb).tskadr);
        READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
    }
    schedule();
//  debug("Returning from tk_sta_tsk\r\n");
    Ok(())
}

// Task Exit API
pub fn tk_ext_tsk() {

    let intsts = di();
    defer! { ei(intsts); }

    // Change state of TCB to dormant
    unsafe {  
        let task: *mut TCB = cur_task;
        if task == core::ptr::null_mut() {
            return;
        }
        (*task).state = TSTAT::TsDormant;
        READY_QUEUE[(*task).itskpri].remove_top();
        schedule();
    }
}

// Task delay API 
pub fn tk_dly_tsk(dlytim: RELTIM) -> Result<(),KernelError>  {

    let intsts = di();
    defer! { ei(intsts); }

    if dlytim <= 0 {
        return Ok(());
    }

    unsafe {
        let task: *mut TCB = cur_task;
        if task == core::ptr::null_mut() {
            return Ok(());
        }
        READY_QUEUE[(*task).itskpri].remove_top();

        // Change state of TCB to wait
        (*task).state = TSTAT::TsWait;
        // Set wait factor
        (*task).waifct = TWFCT::TwfctDly;
        // Set wait time
        (*task).waitim = dlytim + TIMER_PERIOD;
        WAIT_QUEUE.add_entry(task);
    }
    schedule();
    Ok(())
}

// Task sleep API
pub fn tk_slp_tsk(tmout: TMO) -> Result<(),KernelError> {

    let intsts: isize  = di();
    defer! { ei(intsts); }

    // Wake up request exists?
    unsafe {
        let task: *mut TCB = cur_task;
        if task == core::ptr::null_mut() {
            return Ok(());
        }
        if (*task).wupcnt > 0 {
            (*task).wupcnt -= 1;
            return Ok(());
        }
        // no request
        READY_QUEUE[(*task).itskpri].remove_top();

        // Change state of TCB to wait
        (*task).state = TSTAT::TsWait;
        // Set wait factor
        (*task).waifct = TWFCT::TwfctSlp;
        // Set wait time
        if tmout == TMO_FEVR {
            (*task).waitim = tmout;
        } else {
            (*task).waitim = tmout + TIMER_PERIOD;
        }
        WAIT_QUEUE.add_entry(task);
    }
    schedule();
    Ok(())
}

// Task wakeup API
pub fn tk_wup_tsk(tskid: ID) -> Result<(),KernelError> {
    if tskid <= 0 || tskid > CNF_MAX_TSK_ID {
        return Err(KernelError::ID);
    }

    let intsts: isize = di();
    defer! { ei(intsts); }

    // Does task have wait state by tk_slp_tsk() ?
    unsafe {
        let tcb: *mut TCB = core::ptr::addr_of_mut!(TCB_TBL[tskid - 1]);
        if (*tcb).state == TSTAT::TsWait && (*tcb).waifct == TWFCT::TwfctSlp {
            WAIT_QUEUE.remove_entry(tcb);
            (*tcb).state  = TSTAT::TsReady;
            (*tcb).waifct = TWFCT::TwfctNon;
            READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
            schedule();
            return Ok(());
        }
        // Task can execute
        if (*tcb).state == TSTAT::TsReady || (*tcb).state == TSTAT::TsWait {
            // Increase number of wake up request 
            (*tcb).wupcnt += 1;
            return Ok(());
        } else {
            return Err(KernelError::OBJ);
        }
    }
}

// Scheduling task
pub fn schedule() {
//  debug("Enter schedule()\r\n");
    let mut i: usize = 0;
    while i < CNF_MAX_TSK_PRI {
        unsafe {
            if !READY_QUEUE[i].is_empty() {
                 break;
            }
        }
        i += 1;
    }

    unsafe {
        if i < CNF_MAX_TSK_PRI {
 //         debug("alter sche_task\r\n");
            sche_task = READY_QUEUE[i].head;
        } else {
            sche_task = core::ptr::null_mut();
        }

        if sche_task != cur_task && !disp_running {
//          debug("Calling dispatch()\r\n");
            dispatch();
        }
    }
}


