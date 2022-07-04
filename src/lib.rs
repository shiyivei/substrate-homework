use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

//线程池
pub struct ThreadPool {
     //worker vec
     workers: Vec<Worker>,          //对于ThreadPool来说，Worker同 Thread
     sender: mpsc::Sender<Message>, //线程池发送任务给worker
}

//worker，管理线程，从线程池接受任务并执行，使用通道技术
pub struct Worker {
     id: usize,
     //thread::spawn 返回的类型是JoinHandle<T>,此处只处理，无返回值，使用默认的()
     thread: Option<thread::JoinHandle<()>>,
}

//工作,是一个类型别名
type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
     NewJob(Job),
     Terminate,
}
trait FnBox {
     fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
     fn call_box(self: Box<F>) {
          (*self)()
     }
}

impl ThreadPool {
     /// create a new thread pool
     /// the size is the number of threads in the pool

     ///# panics
     /// the new fn will panic if the size is zero

     //关联函数，创建新线程
     pub fn new(size: usize) -> ThreadPool {
          assert!(size > 0);
          //创建通道，通道可以是多个发送者对一个接收者，所以所有worker共享一个receiver，从而在线程间分发任务，引入Arc和Mutex
          let (sender, receiver) = mpsc::channel();
          //重新声明receiver
          let receiver = Arc::new(Mutex::new(receiver));

          //thread 等价于 worker，使用vec存储线程
          let mut workers = Vec::with_capacity(size);

          for id in 0..size {
               //一个job对应一个worker,创建worker时同时把其变为接受者，receiver Arc::clone
               workers.push(Worker::new(id, Arc::clone(&receiver)));
          }
          //赋值，相当于线程池持有通道的发送端
          ThreadPool { workers, sender }
     }

     //处理请求，约束 + 生命周期 同thread::spawn, 后面传入一个闭包，执行逻辑是handle_connection
     pub fn execute<F>(&self, f: F)
     where
          F: FnOnce() + Send + 'static,
     {
          let job = Box::new(f);
          //发送job
          self.sender.send(Message::NewJob(job)).unwrap();
     }
}

//worker 关联函数
impl Worker {
     //使用Arc和Mutex对receiver处理
     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
          //执行
          let thread = thread::spawn(move || loop {
               let message = receiver.lock().unwrap().recv().unwrap();
               //接收并判断
               match message {
                    Message::NewJob(job) => {
                         println!("Worker {} got a job; executing", id);
                         job.call_box()
                    }
                    Message::Terminate => {
                         println!("Worker {} was told to terminate", id);
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

impl Drop for ThreadPool {
     fn drop(&mut self) {
          println!("Sending terminate message to all workers");

          for _ in &mut self.workers {
               self.sender.send(Message::Terminate).unwrap();
          }
          println!("shutting down all workers");

          for worker in &mut self.workers {
               println!("shutting down worker {}", worker.id);

               if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
               }
          }
     }
}
