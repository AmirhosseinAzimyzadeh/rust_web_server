pub struct ThreadPool {
  max_threads: usize,
}

impl ThreadPool {
  pub fn new(max_thread: usize) -> ThreadPool {
    assert!(max_thread > 0);
    ThreadPool { max_threads: max_thread }
  }

  pub fn run<F>(&self, f: F)
  where F: FnOnce() + Send + 'static,
  {

  }
}