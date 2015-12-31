use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Side {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    None = 4
}

impl Side {

    pub fn get_all() -> Vec<Side> {
        vec![
            Side::North,
            Side::East,
            Side::South,
            Side::West
        ]
    }

    pub fn from_i32(i: i32) -> Side {
        match i {
            0 => Side::North,
            1 => Side::East,
            2 => Side::South,
            3 => Side::West,
            4 => Side::None,
            _ => unreachable!()
        }
    }

    pub fn from_offsets(a: &Offset, b: &Offset) -> Side {
        if a.x < b.x {
            Side::East

        } else if a.x > b.x {
            Side::West

        } else if a.y < b.y {
            Side::South

        } else {
            Side::North
        }
    }

    pub fn to_offset(&self) -> Offset {
        match *self {
            Side::North => Offset { x: 0, y: -1 },
            Side::East => Offset { x: 1, y: 0 },
            Side::South => Offset { x: 0, y: 1 },
            Side::West => Offset { x: -1, y: 0 },
            Side::None => Offset { x: 0, y: 0 },
        }
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Offset {
    pub x: i32,
    pub y: i32
}

impl Default for Offset {
    fn default() -> Offset {
        Offset {
            x: 0,
            y: 0
        }
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, other: Offset) -> Offset {
        Offset {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

