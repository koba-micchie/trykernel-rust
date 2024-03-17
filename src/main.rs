#![feature(prelude_2024)]
#![no_std]
#![no_main]

use core::panic;
use core::result::Result::{Ok,Err};


#[path = "apidef.rs" ]
pub mod apidef;
use crate::apidef::*;

#[path = "config.rs" ]
pub mod config;

#[path = "context.rs" ]
pub mod context;

#[path = "error.rs" ]
pub mod error;

#[path = "eventflag.rs" ]
pub mod eventflag;

#[path = "knldef.rs" ]
pub mod knldef;

#[path = "logger.rs" ]
pub mod logger;
use crate::logger::*;

#[path = "semaphore.rs"]
pub mod semaphore;

#[path = "sysdef.rs" ]
pub mod sysdef;

#[path = "syslib.rs" ]
pub mod syslib;
use crate::syslib::*;

#[path = "systimer.rs" ]
pub mod systimer;

#[path = "task.rs" ]
pub mod task;
use crate::task::*;

#[path = "typedef.rs"]
pub mod typedef;
use crate::typedef::*;

#[path = "app/usermain.rs" ]
pub mod usermain;
use crate::usermain::*;

#[path = "boot/boot2.rs" ]
pub mod boot2;

#[path = "boot/reset_handler.rs" ]
pub mod reset_handler;

#[path = "boot/vector_table.rs" ]
pub mod vector_table;


pub static mut TSKSTK_INI: [u8; 1024] = [0; 1024];

fn initsk() {
    debug("Start initsk()\r\n");
    usermain();
    debug("End Try Kernel\r\n");
    debug("Calling tk_ext_tsk\r\n");
    tk_ext_tsk();
}

#[no_mangle]
pub fn main() {
    tm_com_init();
    debug("Start Try Kernel\r\n");
    let tskid_ini: ID ;
    let faddr = initsk;

    let mut ctsk_init = TCtsk {
        tskatr: TA_HLNG | TA_RNG0 | TA_USERBUF,
        task:   faddr as u32,
        itskpri:  1, // priority MAX
        stksz:  core::mem::size_of::<[u8; 1024]>() as u32,
        bufptr: unsafe {(&TSKSTK_INI as *const u8) as u32} ,
    };

    debug("Calling tk_cre_tsk\r\n");
    match tk_cre_tsk(&mut ctsk_init) {
       Ok(id)   => {
          tskid_ini = id;
       }
       Err(err) => {
          error(err) ;
          panic!("failed at tk_cre_tsk");
       }
    };

    debug("Calling tk_sta_tsk\r\n");
    match tk_sta_tsk(tskid_ini) {
       Ok(())      => {}
       Err(err) => {
           error(err);
           panic!("failed at tk_sta_tsk");
       }
    };
}
