//! Client-server library for protobuf message communication over TCP sockets.

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate protobuf;

mod errors;
mod proto;
mod task;

use std::net::{TcpStream, TcpListener, SocketAddr};
use errors::ResultExt;
use task::{Error, MessageLoop, Task};
use proto::message::{Message, Message_MessageType};

/// A client task is an instance of `Task`, a message-handling task with a main
/// loop.
pub struct ClientTask {
    task: Task
}

impl ClientTask {
    pub fn new(stream: TcpStream) -> ClientTask {
        ClientTask {
            task: Task::new(stream)
        }
    }
}

impl MessageLoop for ClientTask {
    fn run(&mut self) {
        let mut request: Message = Message::new();
        request.set_message_type(Message_MessageType::VALUE);
        request.set_root_hash([1, 2, 3, 4, 5, 6].to_vec());
        request.set_value([7, 7, 7].to_vec());
        self.task.send_message(&request).unwrap();

        loop {
            match self.task.receive_message() {
                Ok(message) => self.on_message_received(message).unwrap(),
                Err(Error::ProtobufError(e)) => warn!("Protobuf error {}", e),
                Err(_e) => {
                    warn!("Critical error"); // {}", e);
                    break;
                }
            }
        }
    }

    fn on_message_received(&mut self, message: Message) -> Result<(), Error> {
        match message.get_message_type() {
            Message_MessageType::VALUE => {
                info!("VALUE({:?}, {:?}) received",
                      message.get_root_hash(), message.get_value())
            },
            Message_MessageType::ECHO => {
                info!("ECHO({:?}, {:?}) received",
                      message.get_root_hash(), message.get_value())
            },
            Message_MessageType::READY => {
                info!("READY({:?}) received", message.get_root_hash())
            },
        }
        Ok(())
    }
}

/// A server task is an instance of `Task`, a message-handling task with a main
/// loop.
pub struct ServerTask {
    task: Task
}

impl MessageLoop for ServerTask {
    fn run(&mut self) {
        loop {
            match self.task.receive_message() {
                Ok(message) => self.on_message_received(message).unwrap(),
                Err(Error::ProtobufError(e)) => warn!("Protobuf error {}", e),
                Err(_e) => {
                    warn!("Critical error"); // {}", e);
                    break;
                }
            }
        }
    }

    fn on_message_received(&mut self, message: Message) -> Result<(), Error> {
        match message.get_message_type() {
            Message_MessageType::VALUE => {
                let h = message.get_root_hash();
                let v = message.get_value();
                info!("VALUE({:?}, {:?}) received", h, v);
                let mut response: Message = Message::new();
                response.set_message_type(Message_MessageType::ECHO);
                response.set_root_hash(h.to_vec());
                response.set_value(v.to_vec());
                self.task.send_message(&response)?;
            },
            Message_MessageType::ECHO => {
                info!("ECHO({:?}, {:?}) received",
                      message.get_root_hash(), message.get_value())
            },
            Message_MessageType::READY => {
                info!("READY({:?}) received", message.get_root_hash())
            },
        }
        Ok(())
    }
}

impl ServerTask {
    pub fn new(stream: TcpStream) -> ServerTask {
        ServerTask {
            task: Task::new(stream)
        }
    }
}

/// Starts a server process.
pub fn start_server(addr: &SocketAddr) -> Result<(), errors::Error> {
    let listener = TcpListener::bind(addr).chain_err(|| "Bind failure")?;

    info!("Server started");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection");
                std::thread::spawn(move || {
                    ServerTask::new(stream).run();
                });
            },
            Err(e) => {
                warn!("Failed to connect: {}", e);
            }
        }
    }

    Ok(())
}

/// Starts a client process.
pub fn start_client(addr: &SocketAddr) -> Result<(), errors::Error> {
    let stream = TcpStream::connect(addr).chain_err(|| "Connect failure")?;

    info!("Client started");
    ClientTask::new(stream).run();

    Ok(())
}
