extern crate ws;
extern crate chrono;

use std::sync::{Arc, Mutex, mpsc};

fn hms_string() -> String {
    use chrono::prelude::*;
    let now = Local::now();
    format!("[{}]", now.format("%H:%M:%S"))
}

struct InputFactory {
    tx: mpsc::Sender<String>
}

impl InputFactory {
    fn new(tx: mpsc::Sender<String>) -> Self {
        InputFactory { tx }
    }
}

impl ws::Factory for InputFactory {

    type Handler = InputHandler;

    fn connection_made(&mut self, sender: ws::Sender) -> InputHandler {
        InputHandler {
            sender,
            client_addr: None,
            tx: self.tx.clone()
        }
    }
}

struct InputHandler {
    sender: ws::Sender,
    client_addr: Option<String>,
    tx: mpsc::Sender<String>,
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
            self.tx.send(text.clone()).unwrap();
            if let Some(client_addr) = self.client_addr.clone() {
                println!("{} {} 发送了信息：{}", hms_string(), client_addr, text.clone());
            }
            self.sender.send(ws::Message::Text(format!("You sent: {}", text)))
	    } else {
	    	self.sender.close(ws::CloseCode::Unsupported)
	    }
    }
}

struct OutputFactory {
    handlers: Arc<Mutex<Vec<OutputHandler>>>,
}

impl OutputFactory {
    fn new(rx_ok: mpsc::Receiver<String>) -> Self {
        let ans = OutputFactory {
            handlers: Arc::new(Mutex::new(Vec::new()))
        };
        let handlers = ans.handlers.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = rx_ok.recv() {
                for handler in &*handlers.lock().unwrap() {
                    handler.sender.send(ws::Message::Text(msg.clone())).unwrap();
                }
            }
        });
        ans
    }
}

impl ws::Factory for OutputFactory {

    type Handler = OutputHandler;

    fn connection_made(&mut self, sender: ws::Sender) -> OutputHandler {
        let ans = OutputHandler { sender, client_addr: None };
        self.handlers.lock().unwrap().push(ans.clone());
        ans
    }
}

#[derive(Clone)]
struct OutputHandler {
    sender: ws::Sender,
    client_addr: Option<String>,
}

impl ws::Handler for OutputHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(client_addr) = shake.remote_addr()? {
            self.client_addr = Some(client_addr.clone());
            println!("{} 新的直播页 {} 已连接到服务器！", hms_string(), client_addr);
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, _reason: &str) {
        if let Some(client_addr) = self.client_addr.clone() {
            println!("{} 直播页 {} 断开了连接！代码：{:?}", hms_string(), client_addr, code);
        }
    }
}

fn anti_spam(rx_in: mpsc::Receiver<String>) -> (mpsc::Sender<String>, mpsc::Sender<String>) {
    unimplemented!()
}

fn main() -> ws::Result<()> {
    // 输入 -> tx_in -> rx_in -> AntiSpam -> tx_ok -> rx_ok -> 输出
    //                                    -> tx_err -> rx_err -> 输出
    let (tx_in, rx_in) = mpsc::channel();
    // 1. 从客户端输入弹幕
    // 参会者提交弹幕后，前端通过websocket推送到此处
    let addr = "0.0.0.0:1103";
    std::thread::spawn(move || {
        let factory = InputFactory::new(tx_in);
        ws::WebSocket::new(factory).unwrap()
            .listen(addr.clone()).unwrap();
    });
    println!("{} DanmuRocket客户端监听已启动在 {}！", hms_string(), addr.clone());
    // 2. displayer使用的websocket服务器
    // 将弹幕数据推送到displayer前端
    let addr = "0.0.0.0:11030";
    std::thread::spawn(move || {
        let factory = OutputFactory::new(rx_in);
        ws::WebSocket::new(factory).unwrap()
            .listen(addr.clone()).unwrap();
    });
    println!("{} DanmuRocket显示模块已启动在 {}！", hms_string(), addr.clone());
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