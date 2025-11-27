use serde::{Deserialize, Serialize};

/// Represents the different types of arguments an OSC (Open Sound Control) message can contain.
///
/// This enum covers common OSC argument types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Argument {
    /// An OSC 32-bit integer (`i`).
    Int(i32),
    /// An OSC 32-bit float (`f`).
    Float(f32),
    /// An OSC string (`s`).
    String(String),
    /// An OSC blob (binary data) (`b`).
    Blob(Vec<u8>),
    /// An OSC 64-bit timetag (`t`), usually representing NTP time.
    Timetag(u64),
    // Other types like Double(f64), Char(char), RGBA(u32), Midi(Vec<u8>), etc.,
    // can be added here if needed in the future.
}

// Manual implementation of Eq because f32 doesn't derive Eq.
// PartialEq is already derived and handles f32 comparison appropriately (within tolerance).
// This Eq implementation relies on the PartialEq logic.
impl Eq for Argument {}