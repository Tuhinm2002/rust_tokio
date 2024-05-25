use mini_redis::{client,Result};
pub mod hello_new;
mod spawn_new;
mod process_spawn;
mod some_;
mod tokio_task;
mod tokio_bound;
mod store_stream_val;

#[tokio::main]
async fn main() -> Result<()>{

    let mut client = client::connect("127.0.0.1:6382").await?;
    
    client.set("hello", "world".into()).await?;


    let result = client.get("hello").await?;
    
    println!("The result we got {:?}",result);

    Ok(())
    
}
