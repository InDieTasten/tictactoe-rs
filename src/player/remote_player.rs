use std::net::SocketAddr;

use async_trait::async_trait;
use tokio::{net::{TcpListener, TcpStream}, io::AsyncWriteExt};

use crate::game::{Board, Piece};

use super::Player;

pub struct RemotePlayer {
    piece: Option<Piece>,
    listener: TcpListener,
    socket: TcpStream,
    socket_address: SocketAddr,
}

impl RemotePlayer {
    pub async fn new() -> RemotePlayer {
        let addr = "".to_string();
        let listener = TcpListener::bind(&addr).await.unwrap();
        println!("Waiting for player to connect on {addr}");
        let (mut socket, socket_address) = listener.accept().await.unwrap();

        RemotePlayer {
            piece: None,
            listener,
            socket,
            socket_address,
        }
    }
}

#[async_trait]
impl Player for RemotePlayer {
    fn set_piece(&mut self, piece: crate::game::Piece) {
        self.piece = Some(piece);
    }

    async fn pick_field(&mut self, board: &Board) -> usize {
        let encodedState = bincode::serialize(&(self.piece, board)).unwrap();

        // send current game state to client
        self.socket.write_all(&encodedState).await.expect("failed to write data to socket");

        // get response from client
        self.socket.read_
        return 0;
    }
}
