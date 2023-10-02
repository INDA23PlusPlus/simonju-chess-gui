extern crate chess_network_protocol;

use std::net::{TcpListener, TcpStream};
use ggez::*;
use chess_network_protocol::*;

pub struct Networking {
    listener: Option<TcpListener>,
    stream: Option<TcpStream>,
}

impl Networking {
    pub fn host(&mut self, addr: String) -> GameResult {
        self.disconnect();

        self.listener = Some(TcpListener::bind(addr)?);
        self.stream = match &self.listener {
            Some(x) => Some(x.accept()?.0),
            None => return Err(GameError::CustomError("connection error".to_string())),
        };

        Ok(())
    }

    pub fn connect(&mut self, addr: String) -> GameResult {
        self.disconnect();

        self.stream = Some(TcpStream::connect(addr)?);

        Ok(())
    } 

    pub fn disconnect(&mut self) {
        self.listener = None;
        self.stream = None;
    }
}