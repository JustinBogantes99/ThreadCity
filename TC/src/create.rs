use std::thread;//biblioteca de Hilos 
use std::mem::*;//biblioteca 
//estructura de un Hilo
struct fiber {
    context:u64,
    id:u64,
    active:bool,
    stack:Vec<u64>
}
///Estructura que almena los Scheduler y el hilo
struct thread_info {     
    thread_id:    u64,        
    thread_num:   u64,        
    argv_string: *mut char,       
    Scheduler: String,
}

///cantidad de Hilos que soportaria la aplicacion
static mut MAX_FIBERS: u64 =  10;
//Lista de todos los hilos en funcionamiento
static fiberList:Vec<fiber> = Vec::new();
//argumentos que trabajar los hilos
pub struct FiberArguments {
    pub function: Option<unsafe extern "C" fn() -> ()>,
}
//proceso que deberia iniciar el hilo
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


