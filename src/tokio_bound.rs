use std::vec::Vec;
use tokio::task::yield_now;

#[tokio::main]
async fn main(){
// This bellow line will generate error if we print the value under spawn 
// to avoid that error write all the codes including print statement under a local scope
// btw yield_now is important

// let v:Vec<i32> = vec![1,2,3];


    tokio::spawn( async {
        
        {
            let v:Vec<i32> = vec![1,2,3];
            println!("{:?}",v);

        }

        yield_now().await;

    });

}
