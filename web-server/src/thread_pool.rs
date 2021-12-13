use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct TheadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

// 我们希望通过在所有的 worker 中共享单一 receiver，在线程间分发任务。
// 共享单一 receiver 是为了同一时刻只有一个进程接收到任务，避免多个进程同时消费相同的请求。
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 通过使用 loop 并在循环块之内而不是之外获取锁和任务
                // lock 方法返回的 MutexGuard 在 let job 语句结束之后立刻就被丢弃了。
                // 这确保了 recv 调用过程中持有锁，而在 job() 调用前锁就被释放了，这就允许并发处理多个请求了。
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        // 只共享了 receiver，所以 job 的执行是互不影响的。
                        println!("Worker {} got a job; executing.", id);

                        // 第一个进程在因为 job 执行而阻塞时，其他进程则有机会获得锁。
                        job();
                    },

                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }


            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl TheadPool {

    /// create a thread pool
    /// 
    /// size: the thread's number
    /// 
    /// # Panics
    /// 
    /// * call `new` with a number smaller than zero will panic.
    pub fn new(size: usize) -> TheadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        TheadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for TheadPool {
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
