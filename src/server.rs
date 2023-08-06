use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    sync::{Arc, RwLock},
};

use log::{debug, error, info};

use crate::{header, XmlRpcResult};

struct ServerConfig {
    socket_addr: SocketAddr,
}

pub type Handler = Box<dyn Fn(&str) -> String + Send + Sync + 'static>;
pub type Handlers = Arc<RwLock<HashMap<String, Handler>>>;

pub struct Server {
    tcp_listener: Option<TcpListener>,
    config: ServerConfig,
    handlers: Handlers,
}

impl Server {
    pub fn bind(socket_addr: SocketAddr) -> XmlRpcResult<Server> {
        let tcp_listener = TcpListener::bind(socket_addr)?;
        Ok(Server {
            tcp_listener: Some(tcp_listener),
            config: ServerConfig { socket_addr },
            handlers: Default::default(),
        })
    }

    pub fn run(&mut self) {
        let tcp_listener = self.tcp_listener.take().unwrap();
        info!("listening: {}", self.config.socket_addr);
        accept_loop_tcp(tcp_listener, self.handlers.clone());
    }

    pub fn register_handler(&mut self, path: &str, handler: Handler) {
        self.handlers
            .write()
            .unwrap()
            .insert(path.to_string(), handler);
    }
}

fn accept_loop_tcp(tcp_listener: TcpListener, hanlders: Handlers) {
    let join_handle = std::thread::spawn(move || loop {
        let hanlders = hanlders.clone();
        let accept = tcp_listener.accept();
        match accept {
            Ok((mut stream, remote_addr)) => {
                debug!("a connection accepted: {}", remote_addr);
                let mut buf = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(_size) => {
                        let request = std::str::from_utf8(&buf).unwrap();
                        let response = dispatch_request(request, hanlders);
                        let _ = stream.write_all(response.as_bytes());
                    }
                    Err(e) => {
                        error!("failed read from client: {}", e);
                        let _ = stream.shutdown(std::net::Shutdown::Both);
                    }
                }
            }
            Err(e) => error!("failed to accept connection: {}", e),
        }
    });

    join_handle.join().unwrap();
}

fn dispatch_request(request: &str, hanlders: Handlers) -> String {
    match hanlders.read().unwrap().get("/") {
        Some(handler) => handler(request),
        None => not_found(request),
    }
}

fn not_found(_request: &str) -> String {
    let body = r#"<html>
<head>
  <title>Hello, ArceOS</title>
</head>
<body>
  <center>
    Not Found!!!
  </center>
</body>
</html>
"#;
    format!(header!(), body.len(), body)
}
