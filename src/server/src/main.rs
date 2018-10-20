extern crate ws;

use std::sync::{Arc, Mutex};

fn debug_send(client_addr: ws::Sender, msg: String) {
    println!("[] {:?} 发送了：{}", client_addr, msg);
}

fn debug_new_conn(client_addr: ws::Sender) {
    println!("[] 新的客户端 {:?} 已连接到服务器！", client_addr);
}

fn main() {
    let msg_queue = Arc::new(Mutex::new(vec![String::new(); 0]));
    // 1. 从客户端输入弹幕
    // 参会者提交弹幕后，前端通过websocket推送到此处
    let addr = "0.0.0.0:1103";
    ws::listen(addr, |out| {
        let msg_queue = msg_queue.clone();
        debug_new_conn(out.clone());
		move |msg| {
			if let ws::Message::Text(text) = msg {
                if let Ok(ref mut mutex) = msg_queue.try_lock() {
                    mutex.push(text.clone());
                    debug_send(out.clone(), text.clone());
                    out.send(ws::Message::Text(format!("You sent: {}", text)))
                } else {
                    out.send(ws::Message::Text(format!("Failed to send : {}", text)))
                }
			} else {
				out.close(ws::CloseCode::Unsupported)
			}
		}
    }).unwrap();
    println!("Opened receive websocket at {:?}", addr);
    // 2. displayer使用的websocket服务器
    // 将弹幕数据推送到displayer前端
    loop {

    }

}