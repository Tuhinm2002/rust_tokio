// use std::os::windows::process;

use mini_redis::Connection;
use mini_redis::Frame;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main(){
    // The port where it will listen
    let listner = TcpListener::bind("127.0.0.1:6381").await.unwrap();

    // loop keyword for an infinite loop
    loop 
    {
        let (socket,_) = listner.accept().await.unwrap();
        // adding concurrency
        tokio::spawn(async move {
            socket_process(socket).await;
            
        });
    }

}

async fn socket_process(socket:TcpStream){
    
    let mut connections = Connection::new(socket);

    if let Some(frame_)  = connections.read_frame().await.unwrap(){
        println!("GOT :{:?}",frame_);

        // Error response
        let response = Frame::Error("unimplemented".to_string());
        connections.write_frame(&response).await.unwrap();
    } 
}