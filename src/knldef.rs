use core::prelude::rust_2024::derive;
use core::cmp::PartialEq;

use core::iter::Iterator;

use core::option::Option;
use core::option::Option::{Some,None};

use crate::context::*;
use crate::syslib::*;
use crate::typedef::*;
use crate::error::*;

// タスク状態
#[derive(PartialEq)]
#[derive(Debug)]
pub enum TSTAT {
    // Not exist
    TsNonexist = 0,
    // Executing or Ready to execute 
    TsReady    = 1,
    // Wait 
    TsWait     = 2,
    // Dormant
    TsDormant  = 3,
}

// タスクの待ち要因
#[derive(PartialEq)]
#[derive(Debug)]
pub enum TWFCT {
    // Not exist
    TwfctNon = 0,
    // Wait by tk_dly_tsk
    TwfctDly = 1,
    // Wait wakeup by tk_slp_tsk
    TwfctSlp = 2,
    // Wait flag by tk_wai_flg
    TwfctFlg = 3,
    // Wait resource by tk_wai_sem
    TwfctSem = 4,
}

pub struct TCB {
    // pointer to context information
    pub context: *mut StackFrame,

    pub pre: *mut TCB, 
    pub next: *mut TCB,

    // Status
    pub state: TSTAT,
    // Start address of execution
    pub tskadr:  FP, 
    // priority of execution
    pub itskpri: PRI, 
    // Address on stack 
    pub stkadr: u32, 
    // Size of stack
    pub stksz: SZ,

    // Number of wakeup request 
    pub wupcnt: isize,

    // Wait factor
    pub waifct: TWFCT,
    // Wait time
    pub waitim: RELTIM,
    // Error code for release wait
    pub waierr: KernelError,

    // Pattern of wait flag
    pub waiptn: usize,
    // Wait mode
    pub wfmode: usize,
    // Flag patten for release wait
    pub p_flgptn: usize,

    // Number of resource requested for semaphore 
    pub waisem: isize, 
}

pub struct TcbQueue  {
    pub head: *mut TCB,
}

impl TcbQueue {

    pub fn init() -> Self {
        Self { head: core::ptr::null_mut() }
    }

    pub fn iter(self: &Self) -> TcbQueueIterator {
        TcbQueueIterator { cur: self.head }
    }

    pub fn is_empty(self: &Self) -> bool {
        return self.head == core::ptr::null_mut();
    }

    pub fn add_entry(self: &mut Self, tcb: *mut TCB) {
        // tcbがNULLの場合、何もしない
        if tcb == core::ptr::null_mut() {
            return;
        }

        unsafe {
            // There are more than one task in queue
            if (*self).head != core::ptr::null_mut() {
                // get TCB at end of chain
                let end = (*((*self).head)).pre;
                // add TCB next at end of chain 
                (*end).next = tcb;
                // link chain between end and added TCB 
                (*tcb).pre = end;
                // make head chain to point added TCB 
                (*((*self).head)).pre = tcb;
            } else {
                // If top of queue is empty, register to top 
                (*self).head = tcb;
                // Register itself as end of chain
                (*tcb).pre = tcb;
            }
        }
    }

    pub fn remove_top(self: &mut Self) {
        // Delete top task in queue 
        let head = (*self).head;
        // If queue is empty, not exist to delete
        if head == core::ptr::null_mut() {
            return;
        }

        unsafe {
            // make top points which is next of top
            (*self).head = (*head).next;
            // Confirm to exist next of top 
            // Not exists, need not to maintain last task
            if (*self).head != core::ptr::null_mut()  {
                // Exists, move last task which top hold to pre of next task 
                (*((*self).head)).pre = (*head).pre;
            }
            // re-initialize list chain of dropped TCB 
            (*head).pre = core::ptr::null_mut();
            (*head).next = core::ptr::null_mut();
        }
    }

    pub fn remove_entry(self: &mut Self, tcb: *mut TCB) {
        // Delete specified task from queue 
        // if tcb == NULL, do nothing
        if tcb == core::ptr::null_mut() {
            return;
        }
        // if queue is empty, do nothing
        if (*self).head == core::ptr::null_mut() {
            return;
        }
        // if specified task is top , call remove_top()
        if (*self).head == tcb {
            (*self).remove_top();
        } else {
            unsafe {
                // specified task is not top, move next of prev task to next of itself
                if (*tcb).pre != core::ptr::null_mut() {
                    (*((*tcb).pre)).next = (*tcb).next;
                }
                // confirm specified task are last one
                if (*tcb).next != core::ptr::null_mut() {
                    // not last one, move prev of next task to prev of itself
                    (*((*tcb).next)).pre = (*tcb).pre;
                } else {
                    // last one, move prev of top task to prev of itself
                    (*((*self).head)).pre = (*tcb).pre;
                }
                // re-initialize list chain of dropped TCB
                (*tcb).pre  = core::ptr::null_mut();
                (*tcb).next = core::ptr::null_mut();
            }
        }
    }
}

pub struct TcbQueueIterator  {
    pub cur:  *mut TCB,
}

impl Iterator for TcbQueueIterator {

    type Item = *mut TCB;

    fn next(&mut self) -> Option<Self::Item> {
        let tcb: *mut TCB = self.cur;
        if tcb != core::ptr::null_mut() {
            unsafe {
                (*self).cur = (*tcb).next;
            }
            Some(tcb)
        } else {
            (*self).cur = core::ptr::null_mut();
            None
        }
    }
}

// address of Interrupt Control State Register
const SCB_ICSR: u32 = 0xE000_ED04;
// PendSV set-pending bit
const ICSR_PENDSVSET: u32 = 1 << 28;

// calling dispatch_entry() 
pub fn dispatch() {
    out_w(SCB_ICSR, ICSR_PENDSVSET);
}

// state of kernel object
#[derive(PartialEq)]
pub enum KSSTAT {
    KsNonexist = 1,
    KsExist    = 2,
}

// event flag control block (FLGCB)
pub struct FLGCB  {
    pub state: KSSTAT,
    pub flgptn: usize,
}

// semaphore control block (SEMCB)
pub struct SEMCB {
    // Status of semaphore
    pub state: KSSTAT,
    // Value of semaphore 
    pub semcnt: isize,
    // Max of semaphore 
    pub maxsem: isize,
}

#[test]
fn test_tcb_queue() {

    let mut tcb_ready = TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsReady, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }; 
    let mut tcb_wait = TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsWait, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }; 
    let mut tcb_dormant = TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsDormant, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }; 

    let mut q = TcbQueue { head: core::ptr::null_mut() };
    let p_tcb_ready:   *mut TCB = core::ptr::addr_of_mut!(tcb_ready);
    let p_tcb_wait:    *mut TCB = core::ptr::addr_of_mut!(tcb_wait);
    let p_tcb_dormant: *mut TCB = core::ptr::addr_of_mut!(tcb_dormant);

    q.add_entry(p_tcb_ready);
    q.add_entry(p_tcb_wait);
    q.add_entry(p_tcb_dormant);

    assert_eq!(tcb_ready.state, *(q.head).state);
    assert_eq!(tcb_wait.state,  *(*(q.head).next).state); 
    assert_eq!(tcb_dormant.state, *(*(*(q.head).next).next).state);
    assert_eq!(true,*(*(*(q.head).next).next).next == core::ptr::null_mut()); 
    assert_eq!(*(*(q.head).pre).state, *(*(*(q.head).next).next).state);

    q.remove_top();
    assert_eq!(tcb_wait.state, *(q.head).state);
    assert_eq!(tcb_dormant.state, *(*(q.head).next).state);
    assert_eq!(true,*(*(q.head).next).next == core::ptr::null_mut());
    assert_eq!(tcb_dormant.state, *(*(q.head).pre).state);

    q.remove_entry(p_tcb_dormant);
    assert_eq!(tcb_wait.state, *(q.head).state);
    assert_eq!(true, *(q.head).next == core::ptr::null_mut());
    assert_eq!(tcb_wait.state, *(*(q.head).pre).state);

    q.remove_top();
    assert_eq!(true, *(q.head) == core::ptr::null_mut());
}
