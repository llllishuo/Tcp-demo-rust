
use core::panic;
use std::thread::{self, JoinHandle};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{self, Write};
use std::io::BufRead;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct User{
    pub username: String,
    pub password: String,
    pub is_login: bool,
}
pub fn login(stream: &mut TcpStream) -> User{
    println!("login:");

    let mut username = String::new();
    let mut password = String::new();

    println!("用户名: ");
    io::stdin().read_line(&mut username).expect("username io err...");
    username = username.trim().to_string();
    username.push('\0');
    println!("密码: ");
    io::stdin().read_line(&mut password).expect("password io err...");
    password = password.trim().to_string();
    password.push('\0');


    push_to_server(stream, format!("Enter channel: {}", username).as_str());
    
    User { username, password, is_login: true }
}
pub fn send_msg(stream: &mut TcpStream, user: &mut User) -> Result<(),()>{
    println!("me: ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("无法读取行...");
    buf = buf.trim().to_string();
    if buf == *"exit".to_string(){
        return Err(());
    }
    let msg = format!("{}: {}\0", user.username, buf);

    push_to_server(stream, &msg);
    Ok(())
}
pub fn push_to_server(stream: &mut TcpStream, msg: &str){
    match stream.write(msg.as_bytes()){
        Ok(n) => println!("[Debug] write size {n}"),
        Err(e) => panic!("[Err] {e}"),
    }
    stream.flush();
}
pub fn timer(stream: TcpStream) -> JoinHandle<()>{
    thread::spawn(move||{
        receive(&mut stream.try_clone().expect(""));
    })
}
pub fn receive(stream: &mut TcpStream) -> Result<(),String>{
    let mut reader = io::BufReader::new(stream.try_clone().expect(""));
    let data = reader.fill_buf().expect("").to_vec();
    if data.len() == 0{
        return Ok(());
    }
    let msg = String::from_utf8_lossy(&data);
    println!("\n[Msg] {}\nme:\n", msg.to_string());
    reader.consume(data.len());
    Ok(())
}
pub fn handle_client(stream: TcpStream, cliend_vec: &mut Vec<TcpStream>){ 
    let stream_addr = stream.peer_addr().unwrap();
    loop{
        println!("Information reception ready from {}...", stream_addr);
        let mut reader = io::BufReader::new(stream.try_clone().expect(""));
        let received = reader.fill_buf().expect("").to_vec();
        if received.is_empty() {
            break;
        }
        let msg = String::from_utf8_lossy(&received);
        println!("[Msg] {} (from: {})", msg, stream_addr);
        for cliend in &mut *cliend_vec{
            let cliend_addr = cliend.peer_addr().unwrap();
            if cliend_addr.to_string() == stream_addr.to_string() {
                continue;
            }
            println!("[Debug] {} -> {}", stream_addr, cliend_addr);
            relay(cliend, msg.to_string());
        }
        reader.consume(received.len());
    }
}
pub fn relay(cliend: &mut TcpStream, msg: String){
    let _ = cliend.write(msg.as_bytes());
    let _ = cliend.flush();
}
