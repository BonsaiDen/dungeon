use entity::item::Item;
use entity::trigger::Trigger;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Small,
    Big,
    Boss
}

#[derive(Debug, Eq, PartialEq)]
pub struct Enemy {
    pub typ: Type,
    // TODO How to keep track of item across potential saves?
    pub item: Item,
    pub trigger: Trigger
}

impl Enemy {
    pub fn to_string(&self) -> String {
        format!(
            "E({:?},{:?},{})",
            self.typ,
            self.item,
            self.trigger.to_string()

        ).to_owned()
    }
}

