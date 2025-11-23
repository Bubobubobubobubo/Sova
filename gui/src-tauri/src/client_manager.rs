use anyhow::Result;
use sova_core::server::client::{ClientMessage, SovaClient};
use sova_core::server::ServerMessage;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

#[derive(Clone, serde::Serialize)]
struct ClientDisconnectEvent {
    reason: String,
}

pub struct ClientManager {
    app_handle: AppHandle,
    client: Option<SovaClient>,
    message_sender: Option<mpsc::UnboundedSender<ClientMessage>>,
    message_receiver: Option<mpsc::UnboundedReceiver<ServerMessage>>,
    disconnect_sender: Option<mpsc::UnboundedSender<()>>,
}

impl ClientManager {
    pub fn new(app_handle: AppHandle) -> Self {
        ClientManager {
            app_handle,
            client: None,
            message_sender: None,
            message_receiver: None,
            disconnect_sender: None,
        }
    }

    pub async fn connect(&mut self, ip: String, port: u16) -> Result<()> {
        let mut client = SovaClient::new(ip, port);
        client.connect().await?;

        let (msg_tx, msg_rx) = mpsc::unbounded_channel();
        let (server_tx, server_rx) = mpsc::unbounded_channel();
        let (disconnect_tx, disconnect_rx) = mpsc::unbounded_channel();

        self.spawn_client_task(client, msg_rx, server_tx, disconnect_rx, self.app_handle.clone()).await;

        self.message_sender = Some(msg_tx);
        self.message_receiver = Some(server_rx);
        self.disconnect_sender = Some(disconnect_tx);

        Ok(())
    }

    async fn spawn_client_task(
        &self,
        mut client: SovaClient,
        mut message_receiver: mpsc::UnboundedReceiver<ClientMessage>,
        server_sender: mpsc::UnboundedSender<ServerMessage>,
        mut disconnect_receiver: mpsc::UnboundedReceiver<()>,
        app_handle: AppHandle,
    ) {
        tauri::async_runtime::spawn(async move {
            let mut consecutive_failures = 0;
            loop {
                tokio::select! {
                    Some(message) = message_receiver.recv() => {
                        if let Err(e) = client.send(message).await {
                            eprintln!("Failed to send message: {}", e);
                            let _ = app_handle.emit("client-disconnected", ClientDisconnectEvent {
                                reason: "send_error".to_string(),
                            });
                            return;
                        }
                    }
                    Some(_) = disconnect_receiver.recv() => {
                        eprintln!("Disconnect signal received, closing connection");
                        if let Err(e) = client.disconnect().await {
                            eprintln!("Failed to disconnect client: {}", e);
                        }
                        let _ = app_handle.emit("client-disconnected", ClientDisconnectEvent {
                            reason: "manual_disconnect".to_string(),
                        });
                        return;
                    }
                    read_result = async {
                        // Timeout ready() check to prevent blocking forever on dead connections
                        match tokio::time::timeout(
                            tokio::time::Duration::from_millis(100),
                            client.ready()
                        ).await {
                            Ok(true) => {
                                // Data is available - read it with timeout
                                match tokio::time::timeout(
                                    tokio::time::Duration::from_secs(1),
                                    client.read()
                                ).await {
                                    Ok(result) => result,
                                    Err(_) => Err(std::io::Error::new(
                                        std::io::ErrorKind::TimedOut,
                                        "Read timeout after ready"
                                    ))
                                }
                            }
                            Ok(false) => {
                                // ready() returned false - connection closed by peer
                                Err(std::io::Error::new(
                                    std::io::ErrorKind::ConnectionReset,
                                    "Connection closed"
                                ))
                            }
                            Err(_) => {
                                // ready() timed out - no data available yet (NORMAL during idle)
                                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                                Err(std::io::Error::new(
                                    std::io::ErrorKind::WouldBlock,
                                    "No data available"
                                ))
                            }
                        }
                    } => {
                        match read_result {
                            Ok(message) => {
                                consecutive_failures = 0;
                                if server_sender.send(message).is_err() {
                                    let _ = app_handle.emit("client-disconnected", ClientDisconnectEvent {
                                        reason: "internal_channel_closed".to_string(),
                                    });
                                    return;
                                }
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // No data available - NOT a failure, this is normal during idle
                                // Don't increment consecutive_failures
                            }
                            Err(_) => {
                                // Real error - increment failures
                                consecutive_failures += 1;
                                if consecutive_failures > 100 {  // 100 failures × 10ms = ~1 second
                                    eprintln!("Connection dead after {} failures, disconnecting", consecutive_failures);
                                    if let Err(e) = client.disconnect().await {
                                        eprintln!("Failed to disconnect client: {}", e);
                                    }
                                    let _ = app_handle.emit("client-disconnected", ClientDisconnectEvent {
                                        reason: "connection_lost".to_string(),
                                    });
                                    return;
                                }
                                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn send_message(&self, message: ClientMessage) -> Result<()> {
        if let Some(sender) = &self.message_sender {
            sender.send(message)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Not connected"))
        }
    }

    pub fn try_receive_message(&mut self) -> Option<ServerMessage> {
        if let Some(receiver) = &mut self.message_receiver {
            receiver.try_recv().ok()
        } else {
            None
        }
    }

    pub fn is_connected(&self) -> bool {
        if let Some(sender) = &self.message_sender {
            // Check if the channel is still open (task is still running)
            !sender.is_closed()
        } else {
            false
        }
    }

    pub fn disconnect(&mut self) {
        // Send disconnect signal to the task
        if let Some(disconnect_sender) = &self.disconnect_sender {
            let _ = disconnect_sender.send(());
        }
        
        // Clear all channels
        self.message_sender = None;
        self.message_receiver = None;
        self.disconnect_sender = None;
        self.client = None;
    }
}