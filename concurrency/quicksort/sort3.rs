fn parallel_quicksort<T: PartialOrd + std::marker::Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() <= 2048 {
        quicksort(v);
        return;
    }

    crossbeam::scope(|scope| {
        let mid = partition(v);
        let (lo, hi) = v.split_at_mut(mid);
        scope.spawn(move |_| parallel_quicksort(lo));
        scope.spawn(move |_| parallel_quicksort(hi));
    }).expect("thread spawn failed");
}
