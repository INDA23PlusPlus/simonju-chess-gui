extern crate chess_network_protocol;
extern crate olindba_chess;

use std::{net::{TcpListener, TcpStream}, thread::{Thread, JoinHandle}, sync::mpsc::{Receiver, self, Sender}, fmt::Debug};
use ggez::*;
use chess_network_protocol::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Deserializer, de::IoRead};
use std::thread;

use crate::board::Ply;

mod client;
mod server;

type STC = chess_network_protocol::ServerToClient;
type CTS = chess_network_protocol::ClientToServer;

pub struct ClientGame {
    board: olindba_chess::Game,
    moves: Vec<Ply>,
    game_state: Joever,
    tcp_handler: TcpHandler<STC, CTS>,
}

pub struct ServerGame {
    listener: TcpListener,
    board: olindba_chess::Game,
    tcp_handler: TcpHandler<CTS, STC>,
}

impl ClientGame {
    pub fn new(addr: String) -> GameResult<ClientGame> {
        let stream = TcpStream::connect(addr)?;
        let board = olindba_chess::Game::starting_position();
        let moves = vec![];
        let game_state = Joever::Ongoing;
        let tcp_handler = TcpHandler::new(stream)?;

        Ok(Self {
            board,
            moves,
            game_state,
            tcp_handler,
        })
    }
}

impl ServerGame {
    pub fn new(addr: String) -> GameResult<ServerGame> {
        let listener = TcpListener::bind(addr)?;
        let (stream, _addr) = (&listener).accept()?;
        let board = olindba_chess::Game::starting_position();
        let tcp_handler = TcpHandler::new(stream)?;

        Ok(Self {
            listener,
            board,
            tcp_handler,
        })
    }
}

struct TcpHandler<T, S> where (T, S): Send + DeserializeOwned + Debug + 'static {
    read_receiver: Receiver<T>,
    write_sender: Sender<S>,
    read_handle: JoinHandle<()>,
    write_handle: JoinHandle<()>,
}

impl<T, S> TcpHandler<T, S> 
    where T: Send + DeserializeOwned + Serialize + Debug, S: Send + Serialize + DeserializeOwned + Debug {
    pub fn new(stream: TcpStream) -> Result<Self, GameError> {
        let (read_sender, read_receiver)  = mpsc::channel();

        let (write_sender, write_receiver) = mpsc::channel();

        let stream_clone = stream.try_clone()?;

        let read_handle = thread::spawn(move || (
            Self::read(stream, read_sender)
        ));

        let write_handle = thread::spawn(move || (
            Self::write(stream_clone, write_receiver)
        ));

        Ok(Self {
            read_receiver,
            write_sender,
            read_handle,
            write_handle,
        })
    }

    pub fn read(stream: TcpStream, sender: Sender<T>) {
        loop {
            let mut de = serde_json::Deserializer::from_reader(&stream);

            let deserialized = match T::deserialize(&mut de) {
                Ok(x) => x,
                Err(_) => continue, 
            };

            println!("Received: {:?}", deserialized);

            match sender.send(deserialized) {
                Ok(_) => (),
                Err(_) => continue,
            };
        }
    }

    pub fn write(stream: TcpStream, receiver: Receiver<S>) {
        match receiver.try_recv() {
            Ok(state) => serde_json::to_writer(stream, &state),
            Err(error) => match error {
                mpsc::TryRecvError::Empty => return,
                mpsc::TryRecvError::Disconnected => return, // Todo
            },
        };
    }
}