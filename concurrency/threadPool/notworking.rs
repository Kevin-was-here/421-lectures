use tpool::ThreadPool;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_POOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::new(8));
}

fn parallel_quicksort<T: PartialOrd + std::marker::Send + std::marker::Sync>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() <= PAR_SEQ_THRESHOLD.load(Ordering::SeqCst) as usize {
        quicksort(v);
        return;
    }

    let mid = partition(v);
    // let (lo, hi) = v.split_at_mut(mid);
    let len = v.len();

    let w1 = THREAD_POOL.lock().unwrap().execute(move || { println!("exec: mid {}", mid); } );
    let w2 = THREAD_POOL.lock().unwrap().execute(move || { println!("exec: len {}", len); });
    THREAD_POOL.lock().unwrap().wait(w1);
    THREAD_POOL.lock().unwrap().wait(w2);
}
