use std::thread;

pub struct ThreadPool {
    threads: Vec<Wroker>,
}

#[derive(Debug)]
pub struct PoolCreationError {
    message: String,
}


impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError { message: "invalid size when creating thread pool".to_string()});
        }
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {

        }
        Ok(ThreadPool { threads })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}