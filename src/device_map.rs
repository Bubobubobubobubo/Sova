use std::{collections::HashMap, sync::Mutex};

use crate::{
    clock::{Clock, SyncTime},
    lang::event::{ConcreteEvent, ConcreteEventPayload},
    protocol::{
        self, ProtocolMessage, TimedMessage
    }
};

use protocol::log::LogMessage;

pub struct DeviceMap {
    pub input_connections : Mutex<HashMap<String, Connection>>,
    pub output_connections : Mutex<HashMap<String, Connection>>
}

impl DeviceMap {

    pub fn new() -> Self {
        DeviceMap {
            input_connections : Default::default(),
            output_connections : Default::default()
        }
    }

    pub fn register_input_connection(&self, connection : Connection) {
        self.input_connections.lock().unwrap().insert(connection.id.clone(), connection);
    }

    pub fn register_output_connection(&self, connection : Connection) {
        self.output_connections.lock().unwrap().insert(connection.id.clone(), connection);
    }

    pub fn map_event(&self,
        event : ConcreteEvent,
        date : SyncTime,
        clock : &Clock
    ) -> Vec<TimedMessage> {
        match event.payload {
            ConcreteEventPayload::Nop => Vec::new(),
            ConcreteEventPayload::Chord(_, _) => {
                let msg = serde_json::to_string(&event).unwrap();
                vec![ProtocolMessage::LOG(LogMessage::info(msg)).timed(date)]
            },
            //_ => todo!()
        }
    }

    pub fn find_device(&self, event : &Event) -> &Connection {
        todo!()
    }

}
