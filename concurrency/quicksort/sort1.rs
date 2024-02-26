fn quicksort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    quicksort(lo);
    quicksort(hi);
}

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let hi = v.len() - 1;
    let lo = 0;
    let pivot = hi;

    let mut i = 0;
    for j in lo..hi {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
