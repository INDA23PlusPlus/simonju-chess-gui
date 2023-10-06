extern crate chess_network_protocol;
extern crate olindba_chess;

use std::{net::{TcpListener, TcpStream}, thread::{Thread, JoinHandle}, sync::mpsc::{Receiver, self, Sender}, fmt::Debug};
use ggez::*;
use chess_network_protocol::*;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use serde_json::{Deserializer, de::IoRead};
use std::thread;

use crate::board::{Ply, Board, i_to_x, i_to_y};

mod client;
mod server;

type STC = chess_network_protocol::ServerToClient;
type CTS = chess_network_protocol::ClientToServer;

pub enum Player {
    White,
    Black,
}

pub struct ClientGame {
    player: Player,
    turn: Player,
    board: [[chess_network_protocol::Piece; 8]; 8],
    moves: Vec<Ply>,
    game_state: Joever,
    tcp_handler: TcpHandler<STC, CTS>,
}

pub struct ServerGame {
    player: Player,
    listener: TcpListener,
    board: olindba_chess::Game,
    tcp_handler: TcpHandler<CTS, STC>,
}

impl ClientGame {
    pub fn new(player: Player, addr: String) -> GameResult<ClientGame> {
        // Do handshake stuff
        let turn = Player::White;
        let stream = TcpStream::connect(addr)?;
        let mut de = serde_json::Deserializer::from_reader(&stream);

        let handshake = ClientToServerHandshake {
            server_color: match player {
                Player::White => Color::White,
                Player::Black => Color::Black,
            },
        };
    
        match serde_json::to_writer(&stream, &handshake) {
            Ok(_) => (),
            Err(_) => return Err(GameError::CustomError("Could not write handshake".to_string())),
        }

        let deserialized = match ServerToClientHandshake::deserialize(&mut de)  {
            Ok(x) => x,
            Err(_) => return Err(GameError::CustomError("Could not deserialize handshake".to_string())), 
        };
        println!("Recieved: {:?}", deserialized);

        let board = [[Piece::None; 8]; 8];
        let moves = vec![];
        let game_state = Joever::Ongoing;
        let tcp_handler = TcpHandler::new(stream)?;

        Ok(Self {
            player,
            turn,
            board,
            moves,
            game_state,
            tcp_handler,
        })
    }
}

impl ServerGame {
    pub fn new(port: String) -> GameResult<ServerGame> {
        let listener = TcpListener::bind(port)?;
        let (stream, _addr) = listener.accept()?;

        let mut de = serde_json::Deserializer::from_reader(&stream);
        let deserialized = match ClientToServerHandshake::deserialize(&mut de) {
            Ok(x) => x,
            Err(_) => return Err(GameError::CustomError("Could not deserialize handshake".to_string())), 
        };
        println!("Received: {:?}", deserialized);

        let player = match deserialized.server_color {
            Color::White => Player::White,
            Color::Black => Player::Black,
        };  

        let board = olindba_chess::Game::starting_position();

        let feature_state = vec![
            Features::EnPassant, 
            Features::Castling, 
            Features::Promotion,
            Features::Stalemate,
            Features::PossibleMoveGeneration,
        ];

        let mut board_state = [[Piece::None; 8]; 8];
        for (i, tile) in board.get_board().into_iter().enumerate() {
            board_state[i_to_x(i)][i_to_y(i)] = board.get_board()[i];
        };

        let mut move_state: Vec<chess_network_protocol::Move> = vec![];
        for mv in board.get_moves() {
            move_state.push(Move { 
                start_x: mv.fromx(),
                start_y: mv.fromy(),
                end_x: mv.tox(),
                end_y: mv.toy(),
                promotion: mv.prom(),
            })
        }

        let handshake = ServerToClientHandshake {
            features: feature_state,
            board: board_state,
            moves: move_state,
            joever: Joever::Ongoing,
        };

        match serde_json::to_writer(&stream, &handshake) {
            Ok(_) => (),
            Err(_) => return Err(GameError::CustomError("Could not write handshake".to_string())),
        }

        let tcp_handler = TcpHandler::new(stream)?;

        Ok(Self {
            player,
            listener,
            board,
            tcp_handler,
        })
    }
}

struct TcpHandler<T, S> where (T, S): Send + DeserializeOwned + Serialize + Debug + 'static {
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