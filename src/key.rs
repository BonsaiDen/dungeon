use trigger::Trigger;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Small,
    Boss
}

#[derive(Debug)]
pub struct Key {
    pub trigger: Trigger,
    pub typ: Type
}

impl Key {
    pub fn to_string(&self) -> String {
        let typ = match self.typ {
            Type::Boss => "K:Boss",
            Type::Small => "K:Small"
        };
        format!("{}({})", typ, self.trigger.to_string()).to_string()
    }
}

