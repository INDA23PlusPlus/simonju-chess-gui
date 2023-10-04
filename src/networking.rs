extern crate chess_network_protocol;
extern crate olindba_chess;

use std::net::{TcpListener, TcpStream};
use ggez::*;
use chess_network_protocol::*;
use serde_json::{Deserializer, de::IoRead};

use crate::board::Ply;

mod client;
mod server;

pub struct ClientGame {
    stream: TcpStream,
    board: olindba_chess::Game,
    moves: Vec<Ply>,
    game_state: Joever,
}

pub struct ServerGame {
    listener: TcpListener,
    stream: TcpStream,
    board: olindba_chess::Game,
}

impl ClientGame {
    pub fn new(addr: String) -> GameResult<ClientGame> {
        let stream = TcpStream::connect(addr)?;
        let board = olindba_chess::Game::starting_position();
        let moves = vec![];
        let game_state = Joever::Ongoing;

        Ok(Self {
            stream,
            board,
            moves,
            game_state,
        })
    }
}

impl ServerGame {
    pub fn new(addr: String) -> GameResult<ServerGame> {
        let listener = TcpListener::bind(addr)?;
        let (stream, _addr) = (&listener).accept()?;
        let board = olindba_chess::Game::starting_position();

        Ok(Self {
            listener,
            stream,
            board,
        })
    }
}