use crate::MAX_PACKET;
use crate::packet::{Packet, TRANSFORM_HEADER, SHOOT_HEADER};
use crate::player::Player;
use std::net::SocketAddr;
use sha1::Sha1;
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn handle(buf: [u8; MAX_PACKET], players: &mut HashMap<String, Player>, src: SocketAddr) {
    let packet = match Packet::try_from(buf) {
        Ok(p) => p,
        Err(_) => return,
    };

    match packet.get_typ() {
        TRANSFORM_HEADER => handle_transform(packet, players, src),
        SHOOT_HEADER => handle_shoot(packet, players, src),
        _ => return,
    }
}

fn handle_shoot(packet: Packet, players: &mut HashMap<String, Player>, src: SocketAddr) {
    let player_hash: String = Sha1::from(src.to_string()).digest().to_string();

    if !players.contains_key(player_hash.as_str()) {
        return;
    }
}

fn handle_transform(packet: Packet, players: &mut HashMap<String, Player>, src: SocketAddr) {
    // Get the hash from the addr
    let player_hash: String = Sha1::from(src.to_string()).digest().to_string();

    match players.contains_key(player_hash.as_str()) {
        true => {
            let player = players.get_mut(player_hash.as_str()).unwrap();

            // Update transform of player
            player.transform(
                packet.get_transform().unwrap().0,
                packet.get_transform().unwrap().1,
            );
        },
        false => {
            // Create player
            let player = Player::new(
                src.ip(),
                packet.get_transform().unwrap().0,
                packet.get_transform().unwrap().1,
            );

            println!("Creating new player {}", player_hash.clone());

            players.insert(player_hash, player);
        }
    };
}