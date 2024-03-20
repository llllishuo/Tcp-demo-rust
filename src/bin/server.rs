use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use TcpDemoRust::{handle_client, };


fn main() -> io::Result<()> {
    let mut cliend_vec: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:6188")?;
    let Ok(local_addr) = listener.local_addr()else {todo!()};
    println!("local addr: {}", local_addr);
    for stream in listener.incoming().flatten(){
        let stream_addr = stream.peer_addr().unwrap();
        let cliend_vec = Arc::clone(&cliend_vec);

        std::thread::spawn(move||{
        println!("[Debug] new thread from {}...", stream_addr);
        println!("new stream: {}", stream_addr);
        
        cliend_vec.lock().unwrap().push(stream.try_clone().expect(""));
        println!("[Debug] new receive thread from {}...", stream_addr);
        
        let cliend_vec = &mut cliend_vec.lock().unwrap();
        handle_client(stream, cliend_vec);
        

        println!("[Debug] thread end from {}...", stream_addr);
//        receive_thread.join().unwrap();
        });
    }
    Ok(())
}