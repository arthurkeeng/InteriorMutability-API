use std::thread::{self, JoinHandle};

pub mod random;
pub mod cell;

// pub fn spawn<F, T>(f : F)->JoinHandle<T>
// where  F : FnOnce()-> T + Send + 'static, 
// T : Send + 'static{
    
//     let handle:Vec<JoinHandle<T>>;
//     let new_handle = thread::spawn(f);
//     handle.push(new_handle);
//     handle.join(",")
// }


struct Worker {
    id : usize, 
    handler : JoinHandle<()>
}
pub struct ThreadPool{
    workers : Vec<Worker>
}
impl Worker {
    fn new(id : usize)->Self{
        Worker {
            id , handler : thread::spawn(||{})
        }
    }
}

impl ThreadPool {
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    // create a new thread pool
    // the size is the number of threads in the pool
    // the new function will panic if the size is zero

    pub fn new(size : usize) -> Self{
        assert!(size >0);
        
        let mut workers :Vec<Worker> = Vec::with_capacity(size);
        for _ in 0..size{
            let id = random::random_number(1, 100000);
            // create some threads and store them in the vector
            let new_worker = Worker::new(id as usize);
            workers.push(new_worker);
        }
        ThreadPool{
            workers
        }
    }

    pub fn execute<F>(&self , f : F)
    where F : FnOnce() + Send + 'static{

    }
}