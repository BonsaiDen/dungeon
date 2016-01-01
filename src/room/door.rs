use base::{Side, Offset};
use entity::trigger::Trigger;

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug)]
pub struct Door {
    pub side: Side,
    pub lock: Lock,
    pub to: Offset,
    pub triggers: Vec<Trigger>
}

