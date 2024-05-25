#[allow(unused)]

use tokio::task;

async fn do_something(){

    let handle = tokio::spawn( async {

    "return values";

    });

    let out  =  handle.await.unwrap();
    //println!("GOT {}",out);
}

#[tokio::main]
async fn main(){

    let handle = tokio::spawn(async{"return something"});

    let out = handle.await.unwrap();
    println!("GOT {}",out);
    
    //do_something();

}
