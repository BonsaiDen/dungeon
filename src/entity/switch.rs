use entity::trigger::Trigger;

#[derive(Debug, Eq, PartialEq)]
pub struct Switch {
    pub triggers: Vec<Trigger>
}

impl Switch {
    pub fn to_string(&self) -> String {
        format!("S({:?})", self.triggers).to_owned()
    }
}

