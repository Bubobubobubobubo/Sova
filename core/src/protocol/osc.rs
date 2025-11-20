use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};
use std::fmt;
use std::net::{SocketAddr, UdpSocket};

use crate::protocol::error::ProtocolError;

mod message;
pub use message::*;

mod argument;
pub use argument::Argument;

pub struct OSCOut {
    /// User-defined name to identify this device.
    pub name: String,
    /// The network address (IP and port) for destination OSC messages.
    pub address: SocketAddr,
    /// Estimated network latency (in seconds) used to calculate the timestamp
    /// of sent OSC packets (`OscBundle`).
    pub latency: f64,
    /// The UDP socket used for sending, managed in a thread-safe manner.
    pub socket: Option<UdpSocket>,
}

impl OSCOut {

    pub fn connect(&mut self) -> Result<(), ProtocolError> {
        crate::log_println!(
            "[~] connect() called for OSCOutDevice '{}' @ {}",
            self.name, self.address
        );
        if self.socket.is_some() {
            crate::log_println!("    Already connected.");
            Ok(())
        } else {
            // Bind to any available local port for sending
            let local_addr: SocketAddr = "0.0.0.0:0"
                .parse()
                .expect("Failed to parse local UDP bind address");
            match UdpSocket::bind(local_addr) {
                Ok(udp_socket) => {
                    crate::log_println!(
                        "    Created UDP socket bound to {}",
                        udp_socket.local_addr()?
                    );
                    self.socket = Some(udp_socket);
                    Ok(())
                }
                Err(e) => {
                    crate::log_eprintln!(
                        "[!] Failed to bind UDP socket for OSCOutDevice '{}': {}",
                        self.name, e
                    );
                    Err(ProtocolError::from(e))
                }
            }
        }
    }

    pub fn send(&self, message: OSCMessage) -> Result<(), ProtocolError> {
        if let Some(sock) = &self.socket {
            // Convert our internal OSC Arguments to rosc::OscType arguments
            let rosc_args: Result<Vec<OscType>, rosc::OscError> = message
                .args
                .into_iter()
                .map(|arg| {
                    match arg {
                        Argument::Int(i) => Ok(OscType::Int(i)),
                        Argument::Float(f) => Ok(OscType::Float(f)),
                        Argument::String(s) => Ok(OscType::String(s)),
                        Argument::Blob(b) => Ok(OscType::Blob(b)),
                        Argument::Timetag(t) => Ok(OscType::Time(OscTime {
                            seconds: (t >> 32) as u32,
                            fractional: (t & 0xFFFFFFFF) as u32,
                        })),
                        // ... etc.
                    }
                })
                .collect();
            let rosc_args = rosc_args?; // Propagate potential conversion errors

            let rosc_msg = OscMessage {
                addr: message.addr,
                args: rosc_args,
            };
            let rosc_msg = OscPacket::Message(rosc_msg);

            let packet = if let Some(timetag) = message.timetag {
                // CRITICAL FIX: Calculate OSC Timestamp from target_time, not current time
                // This enables precise OSC bundle timestamping for sample-accurate timing
                let latency_micros = (self.latency * 1_000_000.0) as u64;
                let target_time_micros = timetag + latency_micros;

                // Convert microseconds since UNIX epoch to NTP seconds and fractional parts
                const NTP_UNIX_OFFSET_SECS: u64 = 2_208_988_800; // Offset between 1900 (NTP) and 1970 (Unix)
                let target_time_secs = target_time_micros / 1_000_000;
                let target_micros_remainder = target_time_micros % 1_000_000;
                let ntp_secs = target_time_secs + NTP_UNIX_OFFSET_SECS;
                // Calculate fractional part: (microseconds / 1_000_000.0) * 2^32
                let ntp_frac = ((target_micros_remainder as f64 / 1_000_000.0)
                    * (1u64 << 32) as f64) as u32;

                let osc_time = OscTime {
                    seconds: ntp_secs as u32,
                    fractional: ntp_frac,
                };

                // Create an OSC bundle containing the single message with the calculated timetag
                OscPacket::Bundle(OscBundle {
                    timetag: osc_time,
                    content: vec![rosc_msg],
                })
            } else {
                rosc_msg
            };

            match rosc::encoder::encode(&packet) {
                Ok(buf) => {
                    // Send the encoded buffer to the target address
                    sock.send_to(&buf, self.address).map_err(ProtocolError::from)?; // Convert IO error
                    Ok(())
                }
                Err(e) => Err(ProtocolError::from(e)), // Convert OSC encoding error
            }
        } else {
            Err(ProtocolError(format!(
                "OSC device '{}' socket not connected.",
                self.name
            )))
        }
    }

}

impl fmt::Debug for OSCOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Show socket status (bound/unbound) rather than the object itself
        let socket_status = if self.socket.is_some() {
            "<Bound>"
        } else {
            "<Unbound>"
        };
        f.debug_struct("OSCOutDevice")
            .field("name", &self.name)
            .field("address", &self.address)
            .field("latency", &self.latency)
            .field("socket", &socket_status)
            .finish()
    }
}