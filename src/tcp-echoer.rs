
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

mod tcp_echoer {
  pub fn listen() {
    println!("TCP-ECHOER CALLED");
    // bind the listener to the specified address
    let listener = TcpListener::bind("127.0.0.1:4500");

    // begin listening
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
      match stream {
          Err(e) => { println!("Error occured: {}", e); },
          Ok(stream) => spawn(proc() {
              handle_client(stream)
          })
      }
      drop(acceptor);  // close the socket server
  }

  fn handle_client(mut stream: TcpStream) {
    let message = stream.read_to_end()  // entire stream
      .unwrap()     // unwrapped the result
      .into_ascii() // turn the binary values from decimal to ascii
      .as_slice()   // to string
    println!("message: {}", message);
  }
}

  // accept connections and process them, spawning a new tasks for each one




}
