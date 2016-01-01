use base::{Side, Offset};
use entity::chest::Chest;
use entity::enemy::Enemy;
use entity::switch::Switch;

pub mod door;
pub mod path;
pub use self::path::Path;

/*
use std::ops::{Deref, DerefMut};

use loot::{Chest, Enemy, Switch};

*/

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    None,
    Entrance,
    Exit,
    Hallway,
    Intersection,
    Crossing,
    End,
    Invalid
}

impl Type {
    pub fn to_string(&self) -> String {
        match *self {
            Type::None => "",
            Type::Entrance => "Entrance",
            Type::Exit => "Exit",
            Type::Hallway => "Hallway",
            Type::Intersection => "Intersection",
            Type::Crossing => "Crossway",
            Type::End => "End",
            Type::Invalid => "Invalid"

        }.to_owned()
    }
}

#[derive(Debug)]
pub struct Room {
    pub offset: Offset,
    pub doors: Vec<door::Door>,
    pub typ: Type,
    pub chest: Option<Chest>,
    pub enemy: Option<Enemy>,
    pub switch: Option<Switch>
}

impl Room {

    pub fn new(x: i32, y: i32) -> Room {
        Room {
            offset: Offset {
                x: x,
                y: y
            },
            doors: Vec::new(),
            typ: Type::Invalid,
            chest: None,
            enemy: None,
            switch: None
        }
    }

    pub fn get_door_to_offset_mut(&mut self, to: &Offset) -> Option<&mut door::Door> {

        let side = Side::from_offsets(&self.offset, to);

        self.doors.iter_mut().filter(|d| {
            d.side == side

        }).next()

    }

    pub fn add_door_to(&mut self, other: &Room) {
        self.doors.push(door::Door {
            side: Side::from_offsets(&self.offset, &other.offset),
            lock: door::Lock::None,
            from: self.offset,
            to: other.offset
        })
    }

}

