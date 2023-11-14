use enumset::{EnumSet, EnumSetType};

pub type Triggers = EnumSet<Event>;

#[derive(EnumSetType, Debug)]
pub enum Event {
    NewHighBid,
    NewLowAsk,
}
