use std::{net::SocketAddrV4, sync::{mpsc::Sender, Arc}};

use serde::{Deserialize, Serialize};
use tokio::{io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}, signal};

use crate::{clock::ClockServer, protocol::TimedMessage, schedule::SchedulerMessage};

pub const ENDING_BYTE : u8 = 0x07;

#[derive(Clone)]
pub struct ServerState {
    pub clock_server : Arc<ClockServer>,
    pub world_iface : Sender<TimedMessage>,
    pub sched_iface : Sender<SchedulerMessage>,
}

pub struct BuboCoreServer {
    pub ip : String,
    pub port : u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuboCoreMessage {
    SchedulerControl(SchedulerMessage),
    Success
}

async fn on_message(msg : BuboCoreMessage, state : ServerState) -> BuboCoreMessage {
    match msg {
        BuboCoreMessage::SchedulerControl(sched_msg) => {
            state.sched_iface.send(sched_msg);
            BuboCoreMessage::Success
        },
        _ => BuboCoreMessage::Success
    }
}

async fn process_client(mut socket : TcpStream, state : ServerState) -> io::Result<()> {
    let mut buff = Vec::new();
    loop {
        let mut buf_reader = BufReader::new(socket);
        let n = buf_reader.read_until(ENDING_BYTE, &mut buff).await?;
        socket = buf_reader.into_inner();
        if n == 0 {
            return Ok(());
        }
        buff.pop();
        if let Ok(msg) = serde_json::from_slice::<BuboCoreMessage>(&buff) {
            let res = on_message(msg, state.clone()).await;
            let Ok(res) = serde_json::to_vec(&res) else {
                continue;
            };
            socket.write_all(&res).await?;
        }
        buff.clear();
    }
}

impl BuboCoreServer {

    pub async fn start(&self, state : ServerState) -> io::Result<()> {
        println!("[↕] Starting server");
        let addr = SocketAddrV4::new(self.ip.parse().unwrap(), self.port);
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, c_addr) = tokio::select! {
                _ = signal::ctrl_c() => return Ok(()),
                res = listener.accept() => res.unwrap()
            };
            println!("[🎺] New client connected {}", c_addr);
            let client_state = state.clone();
            tokio::spawn(async move {
                let _ = process_client(socket, client_state).await;
                println!("[👋] Client disconnected {}", c_addr);
            });
        }
    }

}
