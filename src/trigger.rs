#[derive(Debug, Copy, Clone)]
pub enum Trigger {
    None,
    SmallKey,
    BossKey,
    KillEnemies,
    Chest,
    Pot,
    Switch
}

impl Trigger {
    pub fn to_string(&self) -> String {
        match *self {
            Trigger::None => "None",
            Trigger::SmallKey => "SmallKey",
            Trigger::BossKey => "BossKey",
            Trigger::KillEnemies => "Enemy",
            Trigger::Chest => "Chest",
            Trigger::Pot => "Pot",
            Trigger::Switch => "Switch"

        }.to_owned()
    }

    pub fn to_char(&self) -> char {
        match *self {
            Trigger::None => ' ',
            Trigger::SmallKey => 'L',
            Trigger::BossKey => 'B',
            Trigger::KillEnemies => 'E',
            Trigger::Chest => 'C',
            Trigger::Pot => 'P',
            Trigger::Switch => 'S'
        }
    }

}

