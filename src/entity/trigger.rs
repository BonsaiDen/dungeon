use base::Offset;
use entity::item::Item;
use entity::chest::Chest;

#[derive(Debug, Eq, PartialEq)]
pub enum Trigger {
    LockDoor(Offset),
    OpenDoor(Offset),
    Chest(Chest),
    // TODO allow warp creation?
    Item(Item)
}

