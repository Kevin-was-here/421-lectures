struct SleepDuration {
    start: SystemTime,
    elapse: Duration,
    waker: Option<Waker>,
}

struct SleepFuture {
    sleeper: Arc<Mutex<SleepDuration>>
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut sleeper = self.sleeper.lock().unwrap();

        let since = SystemTime::now().duration_since(sleeper.start).unwrap();
        if since >= sleeper.elapse {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that the duration has elapsed.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            sleeper.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl SleepFuture {
    pub fn new(duration: Duration) -> Self {
        let sleeper = Arc::new(Mutex::new(SleepDuration {
            start: SystemTime::now(),
            elapse: duration,
            waker: None,
        }));

        // Spawn the thread that actually sleeps
        let thread_sleeper = sleeper.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut sleeper = thread_sleeper.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            if let Some(waker) = sleeper.waker.take() {
                waker.wake()
            }
        });

        SleepFuture { sleeper }
    }
}
    