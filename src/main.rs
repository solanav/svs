use std::net::UdpSocket;
use crate::handler::handle;
use crate::player::Player;
use std::collections::HashMap;

mod handler;
mod player;
mod packet;
mod types;

const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "4343";
const MAX_PACKET: usize = 256;

fn main() -> std::io::Result<()> {
    let mut players: HashMap<String, Player> = HashMap::new();
    let socket = UdpSocket::bind(format!("{}:{}", SERVER_ADDR, SERVER_PORT))?;

    let mut buf = [0u8; MAX_PACKET];
    loop {
        let (amt, src) = match socket.recv_from(&mut buf) {
            Ok((amt, src)) => (amt, src),
            Err(_) => continue,
        };

        handle(buf, &mut players, src);

        for i in 0..MAX_PACKET {
            buf[i] = 0;
        }
    }
}
