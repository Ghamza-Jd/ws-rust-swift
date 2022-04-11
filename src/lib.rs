extern crate websocket;

use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
use websocket::sync::Client;

#[no_mangle]
pub extern "C" fn swift_ws_new(ip_addr: &str) -> *mut SwiftWS {
    Box::into_raw(Box::new(SwiftWS {
        connection: ip_addr.parse().unwrap(),
        ws_client: None,
        send_loop: None,
        receive_loop: None,
        optional_sender: None
    }))
}

#[no_mangle]
pub extern "C" fn swift_ws_free(ptr: *mut SwiftWS) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn swift_ws_connect(ptr: *mut SwiftWS, protocol: &str) {
    let swift_ws = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    swift_ws.connect(protocol)
}

#[no_mangle]
pub extern "C" fn swift_ws_send_message(ptr: *mut SwiftWS, msg: &str) {
    let swift_ws = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    swift_ws.send_message(msg);
}

#[no_mangle]
pub extern "C" fn swift_ws_close(ptr: *mut SwiftWS) {
    let swift_ws = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    swift_ws.close();
}

pub struct SwiftWS {
    connection: String,
    ws_client: Option<Client<TcpStream>>,
    send_loop: Option<JoinHandle<()>>,
    receive_loop: Option<JoinHandle<()>>,
    optional_sender: Option<Sender<OwnedMessage>>
}

impl SwiftWS {
    fn connect(&mut self, protocol: &str) {
        println!("Connecting to {}", self.connection);
        self.ws_client = Option::from(ClientBuilder::new(&*self.connection)
            .unwrap()
            .add_protocol(protocol)
            .connect_insecure()
            .unwrap());
        println!("Successfully connected");

        let (
            mut receiver,
            mut sender
        ) = self.ws_client.take().unwrap().split().unwrap();

        let (tx, rx) = channel();

        let tx_1 = tx.clone();
        self.optional_sender = Option::from(tx.clone());

        self.send_loop = Option::from(thread::spawn(move || {
            loop {
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        return;
                    }
                    _ => (),
                }
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        }));

        self.receive_loop= Option::from(thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        match tx_1.send(OwnedMessage::Pong(data)) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                return;
                            }
                        }
                    }
                    _ => println!("Receive Loop: {:?}", message),
                }
            }
        }));
    }

    fn send_message(&mut self, msg: &str) {
        let ws_sender = self.optional_sender.take();

        let trimmed = msg.trim();

        let message = match trimmed {
            "/close" => {
                let _ = ws_sender.as_ref().unwrap().send(OwnedMessage::Close(None));
                return;
            }
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match ws_sender.as_ref().unwrap().send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                return;
            }
        }

        self.optional_sender = ws_sender
    }

    fn close(&mut self) {
        println!("Waiting for child threads to exit");

        let _ = self.send_loop.take().unwrap().join();
        let _ = self.receive_loop.take().unwrap().join();

        println!("Exited");
    }
}