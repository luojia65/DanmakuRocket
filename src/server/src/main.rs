extern crate ws;
extern crate chrono;

use std::sync::{Arc, Mutex};

fn hms_string() -> String {
    use chrono::prelude::*;
    let now = Local::now();
    format!("[{}]", now.format("%H:%M:%S"))
}

struct MyHandler {
    sender: ws::Sender,
    client_addr: Option<String>,
    queue: Arc<Mutex<Vec<String>>>,
}

impl ws::Handler for MyHandler {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(client_addr) = shake.remote_addr()? {
            self.client_addr = Some(client_addr.clone());
            println!("{} 新的客户端 {:?} 已连接到服务器！", hms_string(), client_addr);
        }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        println!("{} 客户端 {:?} 断开了连接！", hms_string(), self.client_addr);
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> { 
	    if let ws::Message::Text(text) = msg {
            if let Ok(ref mut mutex) = self.queue.try_lock() {
                mutex.push(text.clone());
                println!("{} 客户端 {:?} 发送了信息：{}", hms_string(), self.client_addr, text.clone());
                self.sender.send(ws::Message::Text(format!("You sent: {}", text)))
            } else {
                self.sender.send(ws::Message::Text(format!("Failed to send : {}", text)))
            }
	    } else {
	    	self.sender.close(ws::CloseCode::Unsupported)
	    }
     }

}

fn main() {
    let msg_queue = Arc::new(Mutex::new(vec![String::new(); 0]));
    // 1. 从客户端输入弹幕
    // 参会者提交弹幕后，前端通过websocket推送到此处
    let addr = "0.0.0.0:1103";
    ws::listen(addr, |out| {
        let msg_queue = msg_queue.clone();
        MyHandler {
            sender: out,
            client_addr: None,
            queue: msg_queue
        }
    }).unwrap();
    println!("Opened receive websocket at {:?}", addr);
    // 2. displayer使用的websocket服务器
    // 将弹幕数据推送到displayer前端
    loop {

    }

}