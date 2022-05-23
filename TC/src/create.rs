use std::thread;
use std::mem::*;

struct fiber {
    context:u64,
    id:u64,
    active:bool,
    stack:Vec<u64>
}

struct thread_info {    /* Used as argument to thread_start() */
    thread_id:    u64,        /* ID returned by pthread_create() */
    thread_num:   u64,       /* Application-defined thread # */
    argv_string: *mut char,      /* From command-line argument */
    Scheduler: String,
}
static mut MAX_FIBERS: u64 =  10;

static fiberList:Vec<fiber> = Vec::new();

pub struct FiberArguments {
    pub function: Option<unsafe extern "C" fn() -> ()>,
}

pub fn initThread(arg:String ){
    println!("{:?}",arg);

    let mut n = 0;

    // Loop while `n` is less than 101
    while n < MAX_FIBERS {
        fiberList[n].id=0;
        n += 1;
    }
}
pub fn createThread(arg:String ){
    println!("{:?}",arg)

}


