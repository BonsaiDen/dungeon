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
    pub triggers: Vec<Trigger>
}

impl Enemy {
    pub fn to_string(&self) -> String {
        format!(
            "E({:?},{:?})",
            self.typ,
            self.triggers

        ).to_owned()
    }
}

