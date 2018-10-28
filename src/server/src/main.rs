extern crate ws;
#[macro_use]
extern crate log;
extern crate env_logger;

use log::LevelFilter;
use env_logger::{Builder, Target};
use std::sync::{Arc, Mutex, mpsc, atomic::{AtomicIsize, Ordering}};
use std::thread;

struct Service {
    
}

impl Service {
    fn new() -> Self {
        Self {}
    }
}

enum StatusSignal {
    ModifyOnlineUserCount(isize)
}

struct InputFactory {
    tx_msg: mpsc::Sender<String>,
    tx_status: mpsc::Sender<StatusSignal>,
}

impl InputFactory {
    fn new(tx_msg: mpsc::Sender<String>, tx_status: mpsc::Sender<StatusSignal>) -> Self {
        InputFactory { tx_msg, tx_status }
    }
}

impl ws::Factory for InputFactory {

    type Handler = InputHandler;

    fn connection_made(&mut self, ws_sender: ws::Sender) -> InputHandler {
        InputHandler {
            ws_sender,
            client_addr: None,
            tx_msg: self.tx_msg.clone(),
            tx_status: self.tx_status.clone(),
        }
    }
}

struct InputHandler {
    ws_sender: ws::Sender,
    client_addr: Option<String>,
    tx_msg: mpsc::Sender<String>,
    tx_status: mpsc::Sender<StatusSignal>,
}

impl ws::Handler for InputHandler {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(client_addr) = shake.remote_addr()? {
            self.client_addr = Some(client_addr.clone());
            self.tx_status.send(StatusSignal::ModifyOnlineUserCount(1)).unwrap();
            info!("新的客户端 {:?} 已连接到服务器！", client_addr);
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, _reason: &str) {
        self.tx_status.send(StatusSignal::ModifyOnlineUserCount(-1)).unwrap();
        if let Some(client_addr) = self.client_addr.clone() {
            info!("客户端 {} 断开了连接！代码：{:?}", client_addr, code);
        }
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> { 
	    if let ws::Message::Text(text) = msg {
            if judge_msg(text.clone()) {
                self.tx_msg.send(text.clone()).unwrap();
                self.ws_sender.send(ws::Message::Text(format!("You sent: {}", text.clone())))?;
                if let Some(client_addr) = self.client_addr.as_ref() {
                    info!("{} 发送了信息：{}", client_addr, text.clone());
                }
            } else {
                self.ws_sender.send(ws::Message::Text(format!("Failed to send: {}", text.clone())))?;
                if let Some(client_addr) = self.client_addr.as_ref() {
                    info!("{} 发送的信息已被拦截：{}", client_addr, text.clone());
                }
            }
	    } else {
	    	self.ws_sender.close(ws::CloseCode::Unsupported)?;
	    }
        Ok(())
    }
}

struct OutputFactory {
    handlers: Arc<Mutex<Vec<OutputHandler>>>,
}

impl OutputFactory {
    fn new(rx_msg: mpsc::Receiver<String>) -> Self {
        let ans = OutputFactory {
            handlers: Arc::new(Mutex::new(Vec::new()))
        };
        let handlers = ans.handlers.clone();
        thread::spawn(move || {
            while let Ok(msg) = rx_msg.recv() {
                for handler in &*handlers.lock().unwrap() {
                    handler.ws_sender.send(ws::Message::Text(msg.clone())).unwrap();
                }
            }
        });
        ans
    }
}

impl ws::Factory for OutputFactory {

    type Handler = OutputHandler;

    fn connection_made(&mut self, ws_sender: ws::Sender) -> OutputHandler {
        let ans = OutputHandler { ws_sender, client_addr: None, family: self.handlers.clone() };
        self.handlers.lock().unwrap().push(ans.clone());
        ans
    }
}

#[derive(Clone)]
struct OutputHandler {
    ws_sender: ws::Sender,
    client_addr: Option<String>,
    family: Arc<Mutex<Vec<OutputHandler>>>,
}

impl ws::Handler for OutputHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(client_addr) = shake.remote_addr()? {
            info!("新的直播页 {} 已连接到服务器！", client_addr);
            self.client_addr = Some(client_addr);
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, _reason: &str) {
        if let Some(client_addr) = self.client_addr.as_ref() {
            info!("直播页 {} 断开了连接！代码：{:?}", client_addr, code);
        }
    }
}

struct BackendFactory {
    handlers: Arc<Mutex<Vec<BackendHandler>>>,
}

impl BackendFactory {
    fn new(rx_status: mpsc::Receiver<StatusSignal>) -> Self {
        let ans = BackendFactory {
            handlers: Arc::new(Mutex::new(Vec::new())),
        };
        let handlers = ans.handlers.clone();
        let client_count = AtomicIsize::new(0);
        thread::spawn(move || {
            while let Ok(signal) = rx_status.recv() {
                // unimplemented!("broadcast signal")
            }
        });
        ans
    }
}

impl ws::Factory for BackendFactory {

    type Handler = BackendHandler;

    fn connection_made(&mut self, ws_sender: ws::Sender) -> BackendHandler {
        let ans = BackendHandler {
            ws_sender,
            client_addr: None,
        };
        self.handlers.lock().unwrap().push(ans.clone());
        ans
    }
}

#[derive(Clone)]
struct BackendHandler {
    ws_sender: ws::Sender,
    client_addr: Option<String>,
}

impl ws::Handler for BackendHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(client_addr) = shake.remote_addr()? {
            info!("新的后台 {} 已连接到服务器！", client_addr);
            self.client_addr = Some(client_addr);
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, _reason: &str) {
        if let Some(client_addr) = self.client_addr.as_ref() {
            info!("后台 {} 断开了连接！代码：{:?}", client_addr, code);
        }
    }
}

fn main() -> ws::Result<()> {
    // 配置logger
    let mut builder = Builder::from_default_env();
    builder.filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    // 开启mpsc
    let (tx_msg, rx_msg) = mpsc::channel();
    let (tx_status, rx_status) = mpsc::channel();
    // 从客户端输入弹幕
    // 参会者提交弹幕后，前端通过websocket推送到此处
    let addr = "0.0.0.0:1103";
    thread::spawn(move || {
        let factory = InputFactory::new(tx_msg, tx_status);
        ws::WebSocket::new(factory).unwrap()
            .listen(addr).unwrap();
    });
    info!("DanmuRocket客户端监听已启动在 {}！", addr);
    // displayer使用的websocket服务器
    // 将弹幕数据推送到displayer前端
    let addr = "0.0.0.0:11030";
    thread::spawn(move || {
        let factory = OutputFactory::new(rx_msg);
        ws::WebSocket::new(factory).unwrap()
            .listen(addr).unwrap();
    });
    info!("DanmuRocket显示模块已启动在 {}！", addr);
    // 后台后端
    let addr = "0.0.0.0:11031";
    thread::spawn(move || {
        let factory = BackendFactory::new(rx_status);
        ws::WebSocket::new(factory).unwrap()
            .listen(addr).unwrap();
    });
    info!("DanmuRocket后台后端已启动在 {}！", addr);
    // 读取控制台输入
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "q" => {    
                info!("正在手动停止DanmuRocket...",);
                std::process::exit(0);
            },
            _ => {}
        }
    }

}

fn judge_msg(mut msg: String) -> bool {
    msg = msg.replace(" ", "");
    if
        msg.contains("script") || 
        msg.contains("&#115;&#99;&#114;&#105;&#112;&#116;") ||
        msg.contains("&#x73;&#x63;&#x72;&#x69;&#x70;&#x74;") ||
        msg.contains("\\x73\\x63\\x72\\x69\\x70\\x74") 
    {
        return false;
    } 
    true
}

