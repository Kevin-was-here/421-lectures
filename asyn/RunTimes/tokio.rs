extern crate tokio;

async fn app() {
    println!("tokio ran me");
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();
    rt.block_on(future);
}

#[tokio::main]
async fn main() {

}