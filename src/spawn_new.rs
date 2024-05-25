// use mini_redis::frame;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use mini_redis::Connection;
use mini_redis::Frame;

#[tokio::main]
async fn main(){
    let listner = TcpListener::bind("127.0.0.1:6380").await.unwrap();

    loop{
        let (socket,_) = listner.accept().await.unwrap();
       process(socket).await; // The function is called from bellow
    }
}

// This function is called above
async fn process(socket:TcpStream){

    let mut connection = Connection::new(socket);

if let Some(frame) = connection.read_frame().await.unwrap() {
    println!("GOT : {:?}",frame);

    let response = Frame::Error("unimplemented".to_string());
    connection.write_frame(&response).await.unwrap();
}

}