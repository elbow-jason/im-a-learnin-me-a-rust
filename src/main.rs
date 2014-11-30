

fn main() {
    println!("Hello, Jason!");
    tcp_echoer::listen_then_send_and_receive()
}



pub mod tcp_echoer {
  use std::io::{TcpListener, TcpStream};
  use std::io::{Acceptor, Listener};
  use std::time::duration::Duration;
  use std::io::timer::sleep;

  pub fn listen_then_send_and_receive() {

    println!("TCP-ECHOER CALLED");
    // bind the listener to the specified address
    let listener = TcpListener::bind("127.0.0.1:4500");

    // begin listening
    let mut acceptor = listener.listen();

    spawn( proc() { the_client_sends_a_delayed_message("Did it work?"); });

    for stream in acceptor.incoming() {
      match stream {
        Err(e) => { println!("Error occured: {}", e); },
        Ok(stream) => { spawn(proc() { echo_the_clients_message(stream); }); }
      }
    }
  }

  fn echo_the_clients_message(mut stream: TcpStream) {
    let message_str = stream
      .read_to_end()
      .unwrap();

    let strthing = String::from_utf8(message_str).unwrap();
    println!("server echos: {}", strthing );
    println!("press ctrl + C to exit...");

    }

  fn the_client_sends_a_delayed_message(msg: &str) {
      // delay for a sec then...
      let delay = Duration::seconds(1);
      sleep(delay);
      println!("client sends: {}", msg);
      // create a mutable unwrapped stream.
      let mut stream = TcpStream::connect("127.0.0.1:4500").unwrap();
      //turn the message to bytes
      let message_bytes = msg.as_bytes();
      //write to the stream then unwrap the success result
      stream.write(message_bytes).unwrap();
  }

}
