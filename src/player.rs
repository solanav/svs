use std::net::IpAddr;
use crate::types::{Vector3, Quaternion};

pub struct Player {
    addr: IpAddr,
    id: String,
    pos: Vector3,
    rot: Quaternion,
}

impl Player {
    pub fn new(addr: IpAddr, hash: String, pos: Vector3, rot: Quaternion) -> Self  {
        Player {
            addr,
            id: String::from("tmp"), // TODO: Hash the address
            pos,
            rot,
        }
    }

    pub fn transform(&mut self, pos: Vector3, rot: Quaternion) {
        self.pos = pos;
        self.rot = rot;
    }
}