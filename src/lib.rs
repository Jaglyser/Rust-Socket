use std::{thread};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()> 
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}


pub struct ThreadPool {
    pool: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, ()> {
        if(size < 1) {
            Err(("Pool size has to be at least 1"))
        } else {
            let workers = (1..size).map(|id| {Worker::new(id)}).collect();
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() -> () + Send + 'static, 
    {
    }
}

