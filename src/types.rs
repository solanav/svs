use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, fmt::Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, fmt::Debug)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl TryFrom<Vec<&str>> for Vector3 {
    type Error = ();

    fn try_from(data: Vec<&str>) -> Result<Self, Self::Error> {
        if data.len() != 3 {
            return Err(());
        }

        let x: f64 = match data[0].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let y: f64 = match data[1].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let z: f64 = match data[2].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        Ok(Vector3 {x, y, z})
    }
}

impl TryFrom<Vec<&str>> for Quaternion {
    type Error = ();

    fn try_from(data: Vec<&str>) -> Result<Self, Self::Error> {
        if data.len() != 4 {
            return Err(());
        }

        let x: f64 = match data[0].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let y: f64 = match data[1].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let z: f64 = match data[2].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let w: f64 = match data[3].parse() {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        Ok(Quaternion {x, y, z, w})
    }
}
