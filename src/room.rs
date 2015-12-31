use std::ops::{Deref, DerefMut};

use base::{Side, Offset};
use trigger::Trigger;
use enemy::Enemy;
use key::Key;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    End,
    Hallway,
    Intersection,
    Crossing,
    Entrance,
    Exit,
    Invalid,
    None
}

impl Type {
    pub fn to_string(&self) -> String {
        match *self {
            Type::End => "End",
            Type::Hallway => "Hallway",
            Type::Intersection => "Intersection",
            Type::Crossing => "Crossway",
            Type::Entrance => "Entrance",
            Type::Exit => "Exit",
            Type::Invalid => "Invalid",
            Type::None => ""

        }.to_owned()
    }
}

#[derive(Debug)]
pub struct Door {
    pub side: Side,
    pub trigger: Trigger,
    pub from: Offset,
    pub to: Offset
}

#[derive(Debug)]
pub struct Room {
    pub offset: Offset,
    pub doors: Vec<Door>,
    pub typ: Type,
    pub enemy: Option<Enemy>,
    pub key: Option<Key>
}

impl Room {

    pub fn new(x: i32, y: i32) -> Room {
        Room {
            offset: Offset {
                x: x,
                y: y
            },
            typ: Type::Invalid,
            key: None,
            enemy: None,
            doors: Vec::new()
        }
    }

    pub fn get_door_to_offset_mut(&mut self, to: &Offset) -> Option<&mut Door> {

        let side = Side::from_offsets(&self.offset, to);

        self.doors.iter_mut().filter(|d| {
            d.side == side

        }).next()

    }

    pub fn add_door_to(&mut self, other: &Room) {
        self.doors.push(Door {
            side: Side::from_offsets(&self.offset, &other.offset),
            trigger: Trigger::None,
            from: self.offset,
            to: other.offset
        })
    }

}

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

