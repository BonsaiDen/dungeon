use base::{Side, Offset};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Lock {
    BossKey,
    SmallKey,
    Trigger,
    None
}

impl Lock {

    pub fn to_char(&self) -> char {
        match *self {
            Lock::BossKey => 'B',
            Lock::SmallKey => 'S',
            Lock::Trigger => 'T',
            Lock::None => ' '
        }
    }

}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Door {
    pub side: Side,
    pub lock: Lock,
    pub from: Offset,
    pub to: Offset
}

