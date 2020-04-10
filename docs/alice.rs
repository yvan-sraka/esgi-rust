// Alice

use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = {
        async move {
          let mut incoming = listener.incoming();
          while let Some(conn) = incoming.next().await {
            match conn {
              Err(e) => eprintln!("accept failed = {:?}", e),
              Ok(mut sock) => {
                // Spawn the future that echos the data and returns how
                // many bytes were copied as a concurrent task.
                tokio::spawn(async move {
                  // Split up the reading and writing parts of the
                  // socket.
                  let (mut reader, mut writer) = sock.split();
      
                  match tokio::io::copy(&mut reader, &mut writer).await {
                    Ok(amt) => {
                      println!("wrote {} bytes", amt);
                    }
                    Err(err) => {
                      eprintln!("IO error {:?}", err);
                    }
                  }
                });
              }
            }
          }
        }
      };

    println!("Server running on localhost:8080");

    // Start the server and block this async fn until `server` spins down.
    server.await;
}
