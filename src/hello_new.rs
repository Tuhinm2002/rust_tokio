use mini_redis::Result;
use mini_redis::client::Client;
// use mini_redis::{client,Result} This way or above way is available

async fn say_hello(){
    println!("Hello world");
}

#[tokio::main]
async fn main(){
    let op = say_hello();
    print!("Hello from main");
    op.await;
}