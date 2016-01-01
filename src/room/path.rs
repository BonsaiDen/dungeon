use std::ops::{Deref, DerefMut};

use base::Offset;

pub struct Path(Vec<Offset>);
pub type ConnectedPath = Vec<(Offset, Offset)>;

impl Path {

    pub fn new() -> Path {
        Path(Vec::new())
    }

    pub fn clone(&self) -> Path {
        Path(self.0.clone())
    }

    pub fn into_connected_path(self) -> ConnectedPath {

        let mut connected = Vec::new();
        if self.len() > 1 {
            for i in 0..self.len() - 1 {
                connected.push((self[i], self[i + 1]));
            }
        }
        connected

    }

}

impl Deref for Path {
    type Target = Vec<Offset>;
    fn deref(&self) -> &Vec<Offset> { &self.0 }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Vec<Offset> { &mut self.0 }
}

