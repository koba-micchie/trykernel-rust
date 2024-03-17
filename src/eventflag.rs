use defer_lite::defer;

use core::result::Result;
use core::result::Result::{Ok,Err};

use crate::apidef::*;
use crate::config::*;
use crate::knldef::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::task::*;
use crate::typedef::*;
use crate::error::*;

// Flag Control Block (FLGCB)
static mut FLGCB_TBL: [ FLGCB; CNF_MAX_FLG_ID ] =
[
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
  FLGCB { state: KSSTAT::KsNonexist, flgptn: 0 },
];
 

// Create Event Flag API 
pub fn tk_cre_flg(pk_cflg: &TCflg) -> Result<ID,KernelError> {

    let intsts = di();
    defer! { ei(intsts);}

    let mut flgid: ID = 0;
    while flgid < CNF_MAX_FLG_ID {
        unsafe {
            if FLGCB_TBL[flgid].state == KSSTAT::KsNonexist {
                break;
            }
        }
        flgid += 1;
    }

    if flgid >= CNF_MAX_FLG_ID {
        return Err(KernelError::LIMIT);
    }
    unsafe {
        FLGCB_TBL[flgid].state = KSSTAT::KsExist;
        FLGCB_TBL[flgid].flgptn = pk_cflg.iflgptn;
    }
    flgid += 1;
    Ok(flgid)
}

// Check condition wait  event flag 
fn check_flag(flgptn: usize, waiptn: usize, wfmode: usize) -> bool {
    if wfmode & TWF_ORW != 0 {
        return (flgptn & waiptn) != 0;
    }
    return (flgptn & waiptn) == waiptn;
}

// Set Event Flag API 
pub fn tk_set_flg(flgid: ID, setptn: usize) ->Result<(),KernelError> {
    if flgid <= 0 || flgid > CNF_MAX_FLG_ID {
        return Err(KernelError::ID);
    }

    let intsts = di();
    defer! { ei(intsts);}

    unsafe {
        let flgcb = core::ptr::addr_of_mut!(FLGCB_TBL[flgid - 1]);
        if (*flgcb).state != KSSTAT::KsExist {
            return Err(KernelError::NOEXS);
        }
        (*flgcb).flgptn |= setptn;
        let mut iter: TcbQueueIterator = WAIT_QUEUE.iter();
        loop {
            let itv = iter.next();
            if !itv.is_some() {
                break;
            }
            let tcb: *mut TCB = itv.unwrap();
            if (*tcb).waifct != TWFCT::TwfctFlg {
                 continue;
            }
            if !check_flag((*flgcb).flgptn, (*tcb).waiptn, (*tcb).wfmode) {
                 continue;
            }
            WAIT_QUEUE.remove_entry(tcb);
            (*tcb).state = TSTAT::TsReady;
            (*tcb).waifct = TWFCT::TwfctNon;
            (*tcb).p_flgptn = (*flgcb).flgptn;
            READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
            schedule();
            if ((*tcb).wfmode & TWF_BITCLR) != 0 {
                // Clear flag 
                (*flgcb).flgptn &= !(*tcb).waiptn;
                if (*flgcb).flgptn == 0 {
                    break;
                }
            }
            if ((*tcb).wfmode & TWF_CLR) != 0 {
                // Clear all flag 
                (*flgcb).flgptn = 0;
                break;
            }
        }
    }
    Ok(())
}

// Event Flag Clear API
pub fn tk_clr_flg(flgid: ID, clrptn: usize) -> Result<(),KernelError> {

    if flgid <= 0 || flgid > CNF_MAX_FLG_ID {
        return Err(KernelError::ID);
    }

    let intsts = di();
    defer! { ei(intsts); }

    unsafe {
        let flgcb = core::ptr::addr_of_mut!(FLGCB_TBL[flgid - 1]);
        if (*flgcb).state != KSSTAT::KsExist {
            return Err(KernelError::NOEXS);
        }
        // Clear flag 
        (*flgcb).flgptn &= clrptn;
    }
    Ok(())
}

// Wait Event Flag API 
pub fn tk_wai_flg(flgid: ID, waiptn: usize, wfmode: usize, p_flgptn: &mut usize, tmout: TMO) -> Result<(),KernelError> {
    if flgid <= 0 || flgid > CNF_MAX_FLG_ID {
        return Err(KernelError::ID);
    }

    let intsts = di();
    defer! { ei(intsts);}

    unsafe {
        let flgcb = core::ptr::addr_of_mut!(FLGCB_TBL[flgid - 1]);
        // Unregistered Event Flag 
        if (*flgcb).state != KSSTAT::KsExist {
             return Err(KernelError::NOEXS);
        }
        // When satisfying wait condition 
        if check_flag((*flgcb).flgptn, waiptn, wfmode) {
            *p_flgptn = (*flgcb).flgptn;
            if (wfmode & TWF_BITCLR) != 0 {
                // Clear flag 
                (*flgcb).flgptn &= !waiptn;
            }
            if (wfmode & TWF_CLR) != 0 {
                // Clear all flag 
                (*flgcb).flgptn = 0;
            }
            return Ok(());
        }
        // Unsatisfying wait condition, and wait time == 0 
        if tmout == TMO_POL {
            return Err(KernelError::TMOUT);
        }
        // Unsatisfying wait condition, migrate to wait state
        let c_task: *mut TCB = cur_task ;
        if c_task == core::ptr::null_mut() {
            return Ok(());
        }
        READY_QUEUE[(*c_task).itskpri].remove_top();
        // Change state of task to wait 
        (*c_task).state = TSTAT::TsWait;
        // Set wait factor 
        (*c_task).waifct = TWFCT::TwfctFlg;
        // Set wait time
        if tmout == TMO_FEVR {
            (*c_task).waitim = tmout;
        } else {
            (*c_task).waitim = tmout + TIMER_PERIOD;
        }
        (*c_task).waiptn = waiptn;
        (*c_task).wfmode = wfmode;
        (*c_task).p_flgptn = *p_flgptn;
        WAIT_QUEUE.add_entry(cur_task);
    }
    schedule();
    Ok(())
}


