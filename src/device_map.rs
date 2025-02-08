use crate::{lang::Event, protocol::ProtocolMessage};

pub struct DeviceMap;

impl DeviceMap {

    pub fn new() -> Self {
        DeviceMap
    }

    pub fn map_event(&self, event : Event) -> ProtocolMessage {
        match event {
            Event::Nop => todo!(),
            Event::Note(_, time_span) => todo!(),
        }
    }

}