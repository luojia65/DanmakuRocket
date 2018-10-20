extern crate ws;
extern crate chrono;

use std::sync::{Arc, Mutex};

fn hms_string() -> String {
    use chrono::prelude::*;
    let now = Local::now();
    format!("[{}]", now.format("%H:%M:%S"))
}

struct InputFactory {
    queue: Arc<Mutex<Vec<String>>>,
}

impl InputFactory {
    fn new(queue: Arc<Mutex<Vec<String>>>) -> Self {
        InputFactory {
            queue
        }
    }
}

impl ws::Factory for InputFactory {

    type Handler = InputHandler;

    fn connection_made(&mut self, sender: ws::Sender) -> InputHandler {
        InputHandler {
            sender,
            client_addr: None,
            queue: self.queue.clone()
        }
    }
}

struct InputHandler {
    sender: ws::Sender,
    client_addr: Option<String>,
    queue: Arc<Mutex<Vec<String>>>,
}

impl ws::Handler for InputHandler {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(client_addr) = shake.remote_addr()? {
            self.client_addr = Some(client_addr.clone());
            println!("{} 新的客户端 {} 已连接到服务器！", hms_string(), client_addr);
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, _reason: &str) {
        if let Some(client_addr) = self.client_addr.clone() {
            println!("{} 客户端 {} 断开了连接！代码：{:?}", hms_string(), client_addr, code);
        }
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> { 
	    if let ws::Message::Text(text) = msg {
            if let Ok(ref mut mutex) = self.queue.try_lock() {
                mutex.push(text.clone());
                if let Some(client_addr) = self.client_addr.clone() {
                    println!("{} {} 发送了信息：{}", hms_string(), client_addr, text.clone());
                }
                self.sender.send(ws::Message::Text(format!("You sent: {}", text)))
            } else {
                self.sender.send(ws::Message::Text(format!("Failed to send : {}", text)))
            }
	    } else {
	    	self.sender.close(ws::CloseCode::Unsupported)
	    }
    }
}

struct OutputFactory {
    queue: Arc<Mutex<Vec<String>>>,
    handlers: Arc<Mutex<Vec<OutputHandler>>>,
}

impl OutputFactory {
    fn new(queue: Arc<Mutex<Vec<String>>>) -> Self {
        OutputFactory {
            queue,
            handlers: Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn broadcast_message(&self) {
        let mut queue = self.queue.lock().unwrap();
        while let Some(msg) = queue.pop() {
            for handler in &*self.handlers.lock().unwrap() {
                handler.sender.send(ws::Message::Text(msg.clone())).unwrap()
            }
        }
    }
}

impl ws::Factory for OutputFactory {

    type Handler = OutputHandler;

    fn connection_made(&mut self, sender: ws::Sender) -> OutputHandler {
        let ans = OutputHandler { sender };
        self.handlers.lock().unwrap().push(ans.clone());
        ans
    }
}

#[derive(Clone)]
struct OutputHandler {
    sender: ws::Sender,
}

impl ws::Handler for OutputHandler {

}

fn main() -> ws::Result<()> {
    let msg_queue = Arc::new(Mutex::new(vec![String::new(); 0]));
    let msg_queue_1 = msg_queue.clone();
    // 1. displayer使用的websocket服务器
    // 将弹幕数据推送到displayer前端
    let addr = "0.0.0.0:11030";
    let mut output_socket = None;
    std::thread::spawn(move || {
        let factory = OutputFactory::new(msg_queue);
        output_socket = Some(Arc::new(Mutex::new(ws::WebSocket::new(factory).unwrap()
            .listen(addr.clone()).unwrap())));
    });

    println!("{} DanmuRocket显示模块已启动在 {}！", hms_string(), addr.clone());
    // 2. 从客户端输入弹幕
    // 参会者提交弹幕后，前端通过websocket推送到此处
    let addr = "0.0.0.0:1103";
    std::thread::spawn(move || {
        let factory = InputFactory::new(msg_queue_1);
        ws::WebSocket::new(factory).unwrap()
            .listen(addr.clone()).unwrap();
    });
    println!("{} DanmuRocket客户端监听已启动在 {}！", hms_string(), addr.clone());
    // 3. 读取控制台
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "q" => {    
                println!("{} 正在手动停止DanmuRocket...", hms_string());
                std::process::exit(0);
            },
            _ => {}
        }
    }

}