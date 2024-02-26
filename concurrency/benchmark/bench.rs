fn reinitialize_data(v: &mut [u32]) {
    let mut rng = rand::thread_rng();
    let len = v.len();
    for i in 0..len {
        v[i] = rng.gen_range(0..len).try_into().unwrap();
    }
}

fn bench<F: FnOnce(&mut [u32])>(sort: F, data: &mut [u32]) -> u64 {
    reinitialize_data(data);

    let start = Instant::now();
    sort(data);
    let elapsed = Instant::now() - start;

    if !data.windows(2).all(|w| w[0] <= w[1]) {
        println!("sort failed!");
    }

    u64::from(elapsed.subsec_micros()) + elapsed.as_secs()*1000000u64
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is the vector length
    let len = match args.get(1) {
        Some(slen) => match slen.parse::<usize>() {
            Ok(num) => num,
            Err(e) => {
                println!("ignoring bad length: {}", e);
                MILLION as usize
            }
        },
        None => MILLION as usize,
    };

    let mut data = initialize_data(len);

    let mut par_mean: f64 = 0.0;
    for run in 0..10 {
        let elapsed = bench(parallel_quicksort, &mut data);
        println!("parallel {}: {} usec", run, elapsed);
        par_mean += elapsed as f64;
    }
    par_mean /= 10.0;
    println!("parallel mean: {} usec", par_mean);

    let mut seq_mean: f64 = 0.0;
    for run in 0..10 {
        let elapsed = bench(quicksort, &mut data);
        println!("sequential {}: {}", run, elapsed);
        seq_mean += elapsed as f64;
    }
    seq_mean /= 10.0;
    println!("sequential mean: {} usec", seq_mean);

    println!("speedup: {}", seq_mean/par_mean);
}