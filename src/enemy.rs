#[derive(Debug, Eq, PartialEq)]
pub enum Enemy {
    Boss,
    Small,
    Big
}

impl Enemy {
    pub fn to_string(&self) -> String {
        match *self {
            Enemy::Boss => "E:Boss",
            Enemy::Small => "E:Small",
            Enemy::Big => "E:Big"

        }.to_owned()
    }
}

