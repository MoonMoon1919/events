use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread)
        }
    }
}


pub struct MessageBus {
    worker: Worker,
    sender: Option<mpsc::Sender<Job>>,
}

impl MessageBus {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let worker = Worker::new(Arc::clone(&receiver));

        MessageBus {
            worker,
            sender: Some(sender)
        }
    }

    pub fn queue<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for MessageBus {
    fn drop(&mut self) {
        drop(self.sender.take());

        if let Some(thread) = self.worker.thread.take() {
            thread.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn some_func(input: String) {
        println!("I am some function with input {}", input)
    }

    fn another_func() {
        println!("I am another function")
    }

    #[test]
    fn test_executing_multiple_funcs() {
        let q = MessageBus::new();

        q.queue(|| {some_func(String::from("hello world"))});
        q.queue(|| {another_func()});
    }
}
