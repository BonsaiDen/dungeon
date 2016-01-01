use entity::trigger::Trigger;

#[derive(Debug, Eq, PartialEq)]
pub struct Switch {
    pub trigger: Trigger
}

impl Switch {
    pub fn to_string(&self) -> String {
        format!("S({})", self.trigger.to_string()).to_owned()
    }
}

