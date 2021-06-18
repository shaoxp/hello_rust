use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new thread pool
    ///
    /// The poolsize is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(poolsize: usize) -> ThreadPool {
        assert!(poolsize > 0);

        let mut workers = Vec::with_capacity(poolsize);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..poolsize {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// excute the f in the thread pool.
    /// it will wait then all thread is busy.

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Message::NewJob(Job::new(Box::new(f))))
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending termiate message!");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutting down all works!!");

        for worker in &mut self.workers {
            println!("shuting down! worker :{}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, recevier: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = recevier.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} get a job. executing.", id);

                    job.exe();
                }
                Message::Terminate => {
                    println!("Worker {} get a terminate signal. exit!.", id);

                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

struct Job {
    pub work: Box<dyn FnOnce() + Send + 'static>,
}

impl Job {
    fn new(w: Box<dyn FnOnce() + Send + 'static>) -> Job {
        Job { work: w }
    }

    fn exe(self) {
        (self.work)()
    }
}
