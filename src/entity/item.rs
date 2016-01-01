#[derive(Debug, Eq, PartialEq)]
pub enum Key {
    Small,
    Boss,
    None
}

#[derive(Debug, Eq, PartialEq)]
pub enum Item {
    Key(Key),
    Compass,
    Map,
    None
}

