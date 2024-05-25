#[allow(unused)]

use mini_redis::{client,Result};

#[tokio::main]
async fn main() -> Result<()>{

    let mut connection = client::connect("127.0.0.1:6379").await?;

    connection.set("hello", "world".into()).await?;

    let res = connection.get("hello").await?;

    println!("The value we got {:?}",res);

    Ok(())
}
