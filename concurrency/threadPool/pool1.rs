type Job = Box<dyn FnOnce() + Send>;

enum Task {
    Quit,
    Work(Job),
}

pub struct Worker {
    id: usize,
    sender: mpsc::Sender<Task>,
    receiver: mpsc::Receiver<bool>,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let (thread_sender, thread_receiver) = mpsc::channel(); 
        let (mgr_sender, mgr_receiver) = mpsc::channel(); 
        let thread = thread::spawn(move || {
            loop {
                let work: Task = thread_receiver.recv().unwrap();
                match work {
                    Task::Quit => {
                        break;
                    },
                    Task::Work(job) => {
                        job();
                    },
                }
                mgr_sender.send(true).expect("failed manager send");
            }
        });

        Worker { id, sender: thread_sender, receiver: mgr_receiver, thread }
    }
}
