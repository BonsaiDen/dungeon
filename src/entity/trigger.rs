use base::Offset;
use entity::item::Key;

#[derive(Debug, Eq, PartialEq)]
pub struct Trigger {
    pub door: Option<Offset>,
    pub key: Option<Key>,
    // Whether this trigger should be stored if a trigger is stored
    // - if the trigger is for enemies, the enemies need to stay defeated, TODO figure out how to
    // keep track of item drops and store them?
    // - if the trigger is for a key, and the key is not yet collected it needs to re-appear
    // - if the trigger is for a door, then the door need to be opened automatically
    pub stored: bool
}

impl Trigger {
    pub fn to_string(&self) -> String {
        format!("T({:?},{:?})", self.door, self.key).to_owned()
    }
}

