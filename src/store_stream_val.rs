#[allow(unused)]

use mini_redis::frame;
use tokio::net::TcpStream;
use mini_redis::Connection;
use mini_redis::Frame;
use tokio::net::TcpListener;
use mini_redis::Command::{self,Get,Set};
use std::collections::HashMap;

#[tokio::main]
async fn main(){

    let listner = TcpListener::bind("127.0.0.1:6382").await.unwrap();

    // use move too not just async alone which will mmove the code to 

    loop{
        let (socket,_) = listner.accept().await.unwrap();


        tokio::task::spawn(async move{
            store_process(socket).await;

        });
    }

}



async fn store_process(socket:TcpStream){

    let mut connection = Connection::new(socket);

    let mut db = HashMap::new();

    while let Some(frame) = connection.read_frame().await.unwrap(){

    let response = match Command::from_frame(frame).unwrap(){
    
        Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
        }

        Get(cmd) => {
        
            if let Some(value) = db.get(cmd.key()){
            
                Frame::Bulk(value.clone().into())

            }
            else {
                Frame::Null
            }

        }
        
        cmd => panic!("unimplemented {:?}",cmd),
    };

    connection.write_frame(&response).await.unwrap();
    
    }

}
