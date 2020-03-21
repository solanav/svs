use std::net::IpAddr;
use crate::types::{Vector3, Quaternion};

pub struct Player {
    addr: IpAddr,
    pos: Vector3,
    rot: Quaternion,
}

impl Player {
    pub fn new(addr: IpAddr, pos: Vector3, rot: Quaternion) -> Self  {
        Player {
            addr,
            pos,
            rot,
        }
    }

    pub fn transform(&mut self, pos: Vector3, rot: Quaternion) {
        self.pos = pos;
        self.rot = rot;
    }
}