async fn my_async() {
    println!("Hello, world!");
}

#[async_std::main]
async fn main() {
    let my_async = my_async();
    my_async.await;
}
