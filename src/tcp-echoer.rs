
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn main() {
  println!("Hello, world! listener");
  let listener = TcpListener::bind("127.0.0.1:4500");

  // bind the listener to the specified address
  let mut acceptor = listener.listen();

  fn handle_client(mut stream: TcpStream) {
    let msg_bytes = stream.read_to_end().unwrap();
    println!("msg_bytes is {}", stream.read_to_end().unwrap());

    //println!("msg_string is {}", msg_bytes.into_ascii() );


  // accept connections and process them, spawning a new tasks for each one
  for stream in acceptor.incoming() {
      match stream {
          Err(e) => { println!("Error occured: {}", e); },
          Ok(stream) => spawn(proc() {
              handle_client(stream)
          })
      }
  }

  // close the socket server
  drop(acceptor);
}
