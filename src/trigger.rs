#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Trigger {
    None,
    Pot,
    Chest,
    Switch,
    SmallKey,
    BossKey,
    Enemies
}

impl Trigger {
    pub fn to_string(&self) -> String {
        match *self {
            Trigger::None => "None",
            Trigger::Pot => "Pot",
            Trigger::Chest => "Chest",
            Trigger::Switch => "Switch",
            Trigger::SmallKey => "SmallKey",
            Trigger::BossKey => "BossKey",
            Trigger::Enemies => "Enemies"

        }.to_owned()
    }

    pub fn to_char(&self) -> char {
        match *self {
            Trigger::None => ' ',
            Trigger::Pot => 'P',
            Trigger::Chest => 'C',
            Trigger::Switch => 'S',
            Trigger::SmallKey => 'L',
            Trigger::BossKey => 'B',
            Trigger::Enemies => 'E'
        }
    }

}

