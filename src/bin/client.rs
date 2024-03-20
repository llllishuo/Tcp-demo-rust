use core::panic;
use std::{net::{
    TcpStream,
}, error::Error, io::{self, Write, BufRead}, thread::{self, JoinHandle}};
use TcpDemoRust::{User, login, timer, send_msg, push_to_server};

fn main() -> Result<(), Box<dyn Error>> {
    let mut user = User{
        username: String::new(), 
        password: String::new(),
        is_login: false
    };
    match TcpStream::connect("127.0.0.1:6188"){
        Ok(mut stream) => {
            println!("TcpStream: {:?}", stream);
            println!("[Notice] You can enter \"exit\" to exit...");
            user = login(&mut stream);
            let cilent = if user.is_login {
                timer(stream.try_clone().expect(""))
            }else {panic!("[Err] timer is err...")};
            while user.is_login {
                match send_msg(&mut stream, &mut user){
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
            cilent.join().unwrap();
            push_to_server(&mut stream, format!("[Notice] Exit Channel: {}", user.username).as_str());
        }
        Err(e) => panic!("[Err] {e}"),
    }

    Ok(())
}
