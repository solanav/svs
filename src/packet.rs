use crate::types::{Vector3, Quaternion};
use crate::MAX_PACKET;
use std::convert::TryFrom;
use std::ops::Deref;

pub const TRANSFORM_HEADER: u8 = 0;
pub const SHOOT_HEADER: u8 = 1;

pub struct Packet {
    typ: u8,
    transform: Option<(Vector3, Quaternion)>,
    shoot: Option<Vector3>,
}

impl Packet {
    pub fn new(typ: u8) -> Self {
        Packet {
            typ,
            transform: None,
            shoot: None,
        }
    }

    pub fn get_typ(&self) -> u8 {
        self.typ
    }

    pub fn get_transform(&self) -> Option<(Vector3, Quaternion)> {
        self.transform.clone()
    }

    pub fn get_shoot(&self) -> Option<Vector3> {
        self.shoot.clone()
    }
}

impl TryFrom<[u8; MAX_PACKET]> for Packet {
    type Error = ();

    fn try_from(buf: [u8; MAX_PACKET]) -> Result<Self, Self::Error> {
        let mut data = String::from_utf8_lossy(&buf)
            .deref()
            .to_string();

        // Remove end bytes
        match data.find('\u{0}') {
            Some(p) => data.truncate(p),
            None => {},
        }

        let mut data: Vec<&str> = data.split(',').collect();

        // Get the type of message
        let typ: u8 = match data[0].parse() {
            Ok(t) => t,
            Err(_) => return Err(()),
        };

        data.remove(0);

        match typ {
            TRANSFORM_HEADER => {
                if data.len() != 7 {
                    println!("Transform data did not have 7 parts");
                    return Err(());
                }

                let pos: Vec<&str> = data.drain(0..3).collect();
                let pos = Vector3::try_from(pos).unwrap();

                let rot: Vec<&str> = data.drain(0..4).collect();
                let rot = Quaternion::try_from(rot).unwrap();

                //println!(">> {:?} : {:?}", pos, rot);

                Ok(Packet {
                    typ,
                    transform: Some((pos, rot)),
                    shoot: None,
                })
            },
            SHOOT_HEADER => {
                if data.len() != 3 {
                    println!("Transform data did not have 3 parts: {:?}", data);
                    return Err(());
                }

                let dir: Vec<&str> = data.drain(0..3).collect();
                let dir = Vector3::try_from(dir).unwrap();

                println!(">> {:?}", dir);

                Ok(Packet {
                    typ,
                    transform: None,
                    shoot: Some(dir),
                })
            },
            _ => Err(())
        }
    }
}