#[allow(unused)]

use tokio::task;
use mini_redis::client;
use mini_redis::Result;
//use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use bytes::Bytes;


type Responder<T> = oneshot::Sender<Result<T>>;

#[derive(Debug)]
enum Command {

    Get {
        Key : String,
        resp : Responder<Option<Bytes>>
    },

    Set {
        
        Key : String,
        Value : Bytes,
        resp : Responder<()>
    }

}


#[tokio::main]
async fn main(){

    //let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    let (tx,mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = task::spawn(async move{
    
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {

            use Command::*;

            match cmd {
            
                Get { Key , resp} => {
                       let res =  client.get(&Key).await;

                       let _ = resp.send(res);
                }

                Set { Key, Value ,resp} => {
        
                    let res = client.set(&Key, Value).await;

                    let _ = resp.send(res);
                }

            }
            
        }

    });

   let t1 =  task::spawn(async move{

       let (resp_tx,resp_rx) = oneshot::channel();

       let cmd = Command::Get { Key: "hello".to_string() , resp : resp_tx };

     tx.send(cmd).await.unwrap();

     let res = resp_rx.await;
     println!("GOT {:?}",res);

    });

    let t2 = task::spawn(async move{

        let (resp_tx,resp_rx) = oneshot::channel();

        let cmd = Command::Set { Key: "hello".to_string(), Value: "world".into(), resp : resp_tx };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT {:?}",res);

    });
   

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();


}
