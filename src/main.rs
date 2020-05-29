use futures_util::AsyncWriteExt;
use ringbahn::Ring;
use std::time::Duration;
use tokio::time::timeout;

async fn do_stuff_async(port: usize) {
    use std::fs::File;
    let file = File::create(format!("foo{}.txt", port)).expect("failed to create file");
    let mut file: Ring<File> = Ring::new(file);
    file.write_all(b"hello, world!").await.unwrap();
}

async fn more_async_work(port: usize) {
    timeout(Duration::from_millis(1), do_stuff_async(port))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    for i in 8000..8003 {
        tokio::spawn(async move { more_async_work(i).await });
    }
}
