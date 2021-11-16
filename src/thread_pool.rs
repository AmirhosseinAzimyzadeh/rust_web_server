use std::{
  thread,
  thread::JoinHandle,
  sync::{
    mpsc,
    Mutex,
    Arc,
  }
};

struct Worker {
  id: usize,
  join_handle: JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let join_handle = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker # {} got a job; executing.", id);

      job();
  });
  Worker { id, join_handle }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(max_thread: usize) -> ThreadPool {
    assert!(max_thread > 0);
    
    let mut workers = Vec::with_capacity(max_thread);
    let (sen, rec) = mpsc::channel();
    let rec = Arc::new(Mutex::new(rec));

    for i in 0..max_thread {
      workers.push(Worker::new(i,Arc::clone(&rec)));
    }

    ThreadPool { workers, sender: sen }
  }

  pub fn run<F>(&self, f: F)
  where F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}