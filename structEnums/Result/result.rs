enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let happy: Result<i32, i32> = Ok(10);
    let sad: Result<i32, i32> = Err(10);

    assert!(happy.is_ok() && !happy.is_err());

    if let Ok(val) = happy {
        println!("The happy value is {val}");
    }

    let rc = match sad {
        Ok(v) => v,
        Err(e) => -e,
    };
    println!("Am I positive about sad? {rc}");
}