use std::sync::{mpsc, Arc, Mutex};

use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> std::io::Result<Self> {
        if size == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "invalid ThreadPool size",
            ));
        }

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone())?)
        }

        Ok(Self {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn shutdown(self) {}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        let workers = std::mem::replace(&mut self.workers, vec![]);
        for mut worker in workers.into_iter() {
            if let Some(thread) = worker.handle.take() {
                println!("Shutting down worker {}", worker.id);
                let _ = thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Self, std::io::Error> {
        let thread = thread::Builder::new()
            .name(format!("Worker(id:{id})"))
            .spawn(move || loop {
                match receiver
                    .lock()
                    .expect("panicked while locking threadpool mutex")
                    .recv()
                {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        eprintln!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            })?;
        Ok(Self {
            id,
            handle: Some(thread),
        })
    }
}
