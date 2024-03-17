
use crate::apidef::*;
use crate::knldef::*;
use crate::sysdef::*;
use crate::task::*;

// handler of systimer interrupt 
#[no_mangle]
pub unsafe extern "C" fn systimer_handler() {
    let mut iter = WAIT_QUEUE.iter();
    loop {
        let itv = iter.next();
        if !itv.is_some() {
            break;
        }
        let tcb: *mut TCB = itv.unwrap();
        if (*tcb).waitim == TMO_FEVR {
            continue;
        }
        if (*tcb).waitim > TIMER_PERIOD {
            // decrease waste time from wait time
            (*tcb).waitim -= TIMER_PERIOD;
        } else {
            WAIT_QUEUE.remove_entry(tcb);
            (*tcb).state = TSTAT::TsReady;
            (*tcb).waifct = TWFCT::TwfctNon;
            READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
        }
    }
    schedule();
}

