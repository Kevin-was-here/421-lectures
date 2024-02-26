const MILLION: u32 = 1024*1024;

static THREAD_COUNT: AtomicU32 = AtomicU32::new(0);
static PAR_SEQ_THRESHOLD: AtomicU32 = AtomicU32::new(2048);

fn parallel_quicksort<T: PartialOrd + std::marker::Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() <= PAR_SEQ_THRESHOLD.load(Ordering::SeqCst) as usize {
        quicksort(v);
        return;
    }

    THREAD_COUNT.fetch_add(2, Ordering::SeqCst);
    
    crossbeam::scope(|scope| {
        let mid = partition(v);
        let (lo, hi) = v.split_at_mut(mid);
        scope.spawn(move |_| parallel_quicksort(lo));
        scope.spawn(move |_| parallel_quicksort(hi));
    }).expect("thread spawn failed");
}
