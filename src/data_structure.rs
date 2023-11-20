use crate::stubs::*;
use crate::Event;
use enumset::{EnumSet, EnumSetType};

pub trait DataStructure {
    fn update(&mut self, dp : &DataPacket) -> EnumSet<Event>;
}