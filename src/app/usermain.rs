use core::panic;
use core::result::Result::{Ok,Err};

use crate::apidef::*;
//use crate::error::*;
use crate::eventflag::*;
use crate::logger::*;
use crate::semaphore::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::task::*;
use crate::typedef::*;

static mut FLGID: ID = 0;

static mut CFLG: TCflg = TCflg {
    flgatr: (TA_TFIFO | TA_WMUL) as u32,
    iflgptn: 0,
};

static mut SEMID: ID = 0;
static mut CSEM: TCsem = TCsem {
    sematr:  (TA_TFIFO | TA_FIRST) as u32,
    isemcnt: 1,
    maxsem:  1,
};

static mut TSKSTK_BTN: [u8; 4096] = [0; 4096];
static mut TSKID_BTN: ID = 0;

// ボタン処理タスク
fn task_btn() {

    debug("Start task_btn\r\n");

    out_w(gpio(13), (in_w(gpio(13)) | GPIO_PUE) & !GPIO_PDE);
    out_w(GPIO_OE_CLR, 1 << 13);
    out_w(gpio_ctrl(13), 5);

    out_w(gpio(14), (in_w(gpio(14)) | GPIO_PUE) & !GPIO_PDE);
    out_w(GPIO_OE_CLR, 1 << 14);
    out_w(gpio_ctrl(14), 5);

    let btn0 = in_w(GPIO_IN) & ((1 << 14) | (1 << 13));

    loop {
        let btn = in_w(GPIO_IN) & ((1 << 14) | (1 << 13));
        let diff = btn ^ btn0;
        if diff != 0 {
            if (diff & (1 << 13)) != 0 && (btn & (1 << 13)) == 0 {
                debug("BTN-1 ON\r\n");
                unsafe {
                    debug("task_btn:Calling tk_set_flg\r\n");
                    match tk_set_flg(FLGID, 1 << 1) {
                        Ok(()) => {}
                        Err(err) => {
                            error(err);
                            panic!("failed at eventflag.tk_set_flg");
                        }
                    }
                }
            }
            if (diff & (1 << 14)) != 0 && (btn & (1 << 14)) == 0 {
                debug("BTN-0 ON\r\n");
                unsafe {
                    debug("task_btn:Calling tk_set_flg\r\n");
                    match tk_set_flg(FLGID, 1 << 0) {
                        Ok(()) => {}
                        Err(err) => {
                            error(err);
                            panic!("failed at eventflag.tk_set_flg");
                        }
                    }
                }
            }
        }
//      debug("task_btn:Calling tk_dly_tsk\r\n");
        match tk_dly_tsk(100) {
            Ok(()) => {}
            Err(err) => {
                error(err);
                panic!("failed at task.tk_dly_tsk");
            }
        }
    }
}

// LED1 処理タスク
static mut TSKSTK_LED1: [u8; 1024] = [0; 1024];
static mut TSKID_LED1: ID = 0;
fn task_led1() {

    debug("Start task_led1\r\n");

    let mut flgptn: usize = 0;
    loop {
        unsafe {
            debug("task_led1:Calling tk_wai_flg\r\n");
            match tk_wai_flg(FLGID, 1 << 0, TWF_ANDW | TWF_BITCLR, &mut flgptn, TMO_FEVR) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at eventflag.tk_wai_flg");
                }
            }
        }
        unsafe {
            debug("task_led1:Calling tk_wai_sem\r\n");
            match tk_wai_sem(SEMID, 1, TMO_FEVR) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at semaphore.tk_wai_sem");
                }
            }
        }

        let mut i: usize = 0;
        while i < 3 {
            out_w(GPIO_OUT_SET, 1 << 25);
            debug("task_led1:Calling tk_dly_tsk\r\n");
            match tk_dly_tsk(500) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at task.tk_dly_tsk");
                }
            }
            out_w(GPIO_OUT_CLR, 1 << 25);
            debug("task_led1:Calling tk_dly_tsk\r\n");
            match tk_dly_tsk(500) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at task.tk_dly_tsk");
                }
            }
            i += 1;
        }
        unsafe { 
            debug("task_led1:Calling tk_sig_sem\r\n");
            match tk_sig_sem(SEMID, 1) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at semaphore.tk_sig_sem");
                }
            }
        }
    }
}

// LED2 処理タスク
static mut TSKSTK_LED2: [u8;1024] = [0;1024];
static mut TSKID_LED2: ID = 0;
fn task_led2() {

    debug("Start task_led2\r\n");

    let mut flgptn: usize = 0;

    loop {
        unsafe {
            debug("task_led2:Calling tk_wai_flg\r\n");
            match tk_wai_flg(FLGID, 1 << 1, TWF_ANDW | TWF_BITCLR, &mut flgptn, TMO_FEVR) { 
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at eventflag.tk_wai_flg");
                }
            }
            debug("task_led2:Calling tk_wai_sem\r\n");
            match tk_wai_sem(SEMID, 1, TMO_FEVR) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at semaphore.tk_wai_sem");
                }
            }
        }

        let mut i: usize = 0;
        while i < 5 {
            out_w(GPIO_OUT_SET, 1 << 25);
            debug("task_led2:Calling tk_dly_tsk\r\n");
            match tk_dly_tsk(100) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at tk_dly_tsk");
                }
            }
            out_w(GPIO_OUT_CLR, 1 << 25);
            debug("task_led2:Calling tk_dly_tsk\r\n");
            match tk_dly_tsk(100) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at tk_dly_tsk");
                }
            };
            i += 1;
        }
        unsafe {
            debug("task_led2:Calling tk_sig_sem\r\n");
            match tk_sig_sem(SEMID, 1) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("Failed at semaphore.tk_dly_sem");
                }
            }
        }
    }
}


fn main()  {
    debug("Start app main()\r\n");

    static mut CTSK_BTN: TCtsk = TCtsk {
        tskatr:  TA_HLNG | TA_RNG3 | TA_USERBUF,
        task: 0,
        itskpri:  10,
        stksz:  core::mem::size_of::<[u8;4096]>() as u32,
        bufptr: 0,
    };
    static mut CTSK_LED1: TCtsk = TCtsk {
        tskatr: TA_HLNG | TA_RNG3 | TA_USERBUF,
        task:  0,
        itskpri: 10,
        stksz:  core::mem::size_of::<[u8;1024]>() as u32,
        bufptr: 0,
    };
    static mut CTSK_LED2: TCtsk = TCtsk {
        tskatr: TA_HLNG | TA_RNG3 | TA_USERBUF,
        task: 0,
        itskpri:  10,
        stksz:  core::mem::size_of::<[u8;1024]>() as u32,
        bufptr: 0,
    };

    unsafe {
        let faddr = task_btn;
        CTSK_BTN.task = faddr as u32;
        CTSK_BTN.bufptr = (&TSKSTK_BTN as *const u8) as u32;
        let faddr = task_led1;
        CTSK_LED1.task = faddr as u32;
        CTSK_LED1.bufptr = (&TSKSTK_LED1 as *const u8) as u32;
        let faddr = task_led2;
        CTSK_LED2.task = faddr as u32;
        CTSK_LED2.bufptr = (&TSKSTK_LED2 as *const u8) as u32;
    }

    unsafe {
        debug("appmain:Calling tk_cre_flg()\r\n");
        match tk_cre_flg(&CFLG) {
            Ok(id) => {
                FLGID = id;
            }
            Err(err) => {
                error(err);
                panic!("Failed at eventflag.tk_cre_flg");
            }
        }
        debug("appmain:Calling tk_cre_sem()\r\n");
        match tk_cre_sem(&CSEM) {
            Ok(id) => {
                SEMID = id;
            }
            Err(err) => {
                error(err);
                panic!("Failed at semaphore.tk_cre_sem");
            }
        }
        debug("appmain:Calling tk_cre_tsk()\r\n");
        match tk_cre_tsk(&mut CTSK_BTN) {
            Ok(id) => {
                TSKID_BTN = id;
            }
            Err(err) => {
                error(err);
                panic!("Failed at task.tk_cre_tsk");
            }
        }
        debug("appmain:Calling tk_sta_tsk()\r\n");
        match tk_sta_tsk(TSKID_BTN) {
            Ok(()) => {}
            Err(err) => {
                error(err);
                panic!("Failed at task.tk_sta_tsk");
            }
        }
        debug("appmain:Calling tk_cre_tsk()\r\n");
        match tk_cre_tsk(&mut CTSK_LED1) {
            Ok(id) => {
                TSKID_LED1 = id;
            }
            Err(err) => {
                error(err);
                panic!("Failed at task.tk_cre_tsk");
            }
        } 
        debug("appmain:Calling tk_sta_tsk()\r\n");
        match tk_sta_tsk(TSKID_LED1) {
            Ok(()) => {}
            Err(err) => {
                error(err);
                panic!("Failed at task.tk_sta_tsk");
            }
        }
        debug("appmain:Calling tk_cre_tsk()\r\n");
        match tk_cre_tsk(&mut CTSK_LED2) {
            Ok(id) => {
                TSKID_LED2 = id;
            }
            Err(err) => {
                error(err);
                panic!("Failed at task.tk_cre_tsk");
            }
        }     
        debug("appmain:Calling tk_sta_tsk()\r\n");
        match tk_sta_tsk(TSKID_LED2) {
            Ok(()) => {}
            Err(err) => {
                error(err);
                panic!("Failed at task.tk_sta_tsk");
            }
        }
    }
}

pub fn usermain() {
        debug("Start app usermain()\r\n");
        main();
}
