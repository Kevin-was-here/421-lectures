#[async_std::main]
async fn main() {
    join!(
        async {
            for i in 1..=5 {
                println!("Sleeping {}", i);
                let sleeper = SleepFuture::new(Duration::from_millis(1000)); //call back to the runtime
                sleeper.await;
            }
            AlwaysReady
        },
        async { 
            for i in 1..=10 {
                println!("Interrupting {}", i);
                let sleeper = SleepFuture::new(Duration::from_millis(500));
                sleeper.await;
            }
            AlwaysReady
        }
    );
}
