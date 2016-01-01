use entity::item::Item;

#[derive(Debug, Eq, PartialEq)]
pub struct Chest {
    pub item: Item
}

impl Chest {
    pub fn to_string(&self) -> String {
        format!("C({:?})", self.item).to_owned()
    }
}

