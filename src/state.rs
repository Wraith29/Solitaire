use crate::card::Card;

pub struct State {
    pub selected_card: Option<Card>,
}

impl State {
    pub fn new() -> State {
        State {
            selected_card: None,
        }
    }
}
