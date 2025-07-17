use std::sync::{Arc, Mutex, OnceLock};
use std::io::Write;
use crossbeam_channel::{Sender, Receiver, unbounded};
use tokio::sync::watch;
use crate::protocol::log::{LogMessage, Severity};
use crate::protocol::message::{TimedMessage, ProtocolMessage};
use crate::protocol::payload::ProtocolPayload;
use crate::protocol::device::ProtocolDevice;
use crate::schedule::notification::SchedulerNotification;


/// Global logger instance
static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// Logger operating mode
#[derive(Debug, Clone)]
pub enum LoggerMode {
    /// Standalone mode: logs directly to terminal only
    Standalone,
    /// Embedded mode: logs through channel communication (legacy)
    Embedded(Sender<LogMessage>),
    /// Network mode: logs to clients via notification system (no terminal)
    Network(watch::Sender<SchedulerNotification>),
    /// Dual mode: logs to terminal AND sends to clients (preferred for servers)
    Dual(watch::Sender<SchedulerNotification>),
}

/// Core logging system that supports both standalone and embedded modes
pub struct Logger {
    mode: Arc<Mutex<LoggerMode>>,
}

impl Logger {
    /// Create a new logger in standalone mode
    pub fn new_standalone() -> Self {
        Logger {
            mode: Arc::new(Mutex::new(LoggerMode::Standalone)),
        }
    }

    /// Create a new logger in embedded mode with a channel sender
    pub fn new_embedded(sender: Sender<LogMessage>) -> Self {
        Logger {
            mode: Arc::new(Mutex::new(LoggerMode::Embedded(sender))),
        }
    }

    /// Create a new logger in network mode with a notification sender
    pub fn new_network(sender: watch::Sender<SchedulerNotification>) -> Self {
        Logger {
            mode: Arc::new(Mutex::new(LoggerMode::Network(sender))),
        }
    }

    /// Switch to embedded mode with the provided channel sender
    pub fn set_embedded_mode(&self, sender: Sender<LogMessage>) {
        if let Ok(mut mode) = self.mode.lock() {
            *mode = LoggerMode::Embedded(sender);
        }
    }

    /// Switch to network mode with the provided notification sender
    pub fn set_network_mode(&self, sender: watch::Sender<SchedulerNotification>) {
        if let Ok(mut mode) = self.mode.lock() {
            *mode = LoggerMode::Network(sender);
        }
    }

    /// Switch to dual mode (terminal + network) with the provided notification sender
    pub fn set_dual_mode(&self, sender: watch::Sender<SchedulerNotification>) {
        if let Ok(mut mode) = self.mode.lock() {
            *mode = LoggerMode::Dual(sender);
        }
    }

    /// Switch to standalone mode
    pub fn set_standalone_mode(&self) {
        if let Ok(mut mode) = self.mode.lock() {
            *mode = LoggerMode::Standalone;
        }
    }

    /// Log a message with the specified severity
    pub fn log(&self, level: Severity, msg: String) {
        let log_msg = LogMessage::new(level, msg);
        
        if let Ok(mode) = self.mode.lock() {
            match &*mode {
                LoggerMode::Standalone => {
                    match log_msg.level {
                        Severity::Fatal | Severity::Error => {
                            eprintln!("{}", log_msg);
                            let _ = std::io::stderr().flush();
                        }
                        _ => {
                            println!("{}", log_msg);
                            let _ = std::io::stdout().flush();
                        }
                    }
                }
                LoggerMode::Embedded(sender) => {
                    if let Err(_) = sender.try_send(log_msg.clone()) {
                        // Fallback to terminal if channel is full/closed
                        eprintln!("Logger channel error: {}", log_msg);
                    }
                }
                LoggerMode::Network(sender) => {
                    // Wrap the LogMessage in a TimedMessage for the notification system
                    let timed_message = TimedMessage {
                        message: ProtocolMessage {
                            device: Arc::new(ProtocolDevice::Log),
                            payload: ProtocolPayload::LOG(log_msg.clone()),
                        },
                        time: 0, // Immediate execution
                    };
                    let notification = SchedulerNotification::Log(timed_message);
                    if let Err(_) = sender.send(notification) {
                        // Fallback to terminal if notification channel is closed
                        eprintln!("Logger notification error: {}", log_msg);
                    }
                }
                LoggerMode::Dual(sender) => {
                    // ALWAYS log to terminal first (essential for standalone debugging)
                    match log_msg.level {
                        Severity::Fatal | Severity::Error => {
                            eprintln!("{}", log_msg);
                            let _ = std::io::stderr().flush();
                        }
                        _ => {
                            println!("{}", log_msg);
                            let _ = std::io::stdout().flush();
                        }
                    }
                    
                    // ALWAYS try to send to clients (but don't block if failed)
                    let timed_message = TimedMessage {
                        message: ProtocolMessage {
                            device: Arc::new(ProtocolDevice::Log),
                            payload: ProtocolPayload::LOG(log_msg.clone()),
                        },
                        time: 0, // Immediate execution
                    };
                    let notification = SchedulerNotification::Log(timed_message);
                    // Explicitly ignore errors - terminal logging is the fallback
                    let _ = sender.send(notification);
                }
            }
        }
    }

    /// Log with debug severity
    pub fn debug(&self, msg: String) {
        self.log(Severity::Debug, msg);
    }

    /// Log with info severity
    pub fn info(&self, msg: String) {
        self.log(Severity::Info, msg);
    }

    /// Log with warn severity
    pub fn warn(&self, msg: String) {
        self.log(Severity::Warn, msg);
    }

    /// Log with error severity
    pub fn error(&self, msg: String) {
        self.log(Severity::Error, msg);
    }

    /// Log with fatal severity
    pub fn fatal(&self, msg: String) {
        self.log(Severity::Fatal, msg);
    }
}

/// Initialize the global logger in standalone mode
pub fn init_standalone() {
    let _ = GLOBAL_LOGGER.set(Logger::new_standalone());
}

/// Initialize the global logger in embedded mode
pub fn init_embedded(sender: Sender<LogMessage>) {
    let _ = GLOBAL_LOGGER.set(Logger::new_embedded(sender));
}

/// Initialize the global logger in network mode
pub fn init_network(sender: watch::Sender<SchedulerNotification>) {
    let _ = GLOBAL_LOGGER.set(Logger::new_network(sender));
}

/// Create a logging channel pair
pub fn create_log_channel() -> (Sender<LogMessage>, Receiver<LogMessage>) {
    unbounded()
}

/// Get the global logger instance
pub fn get_logger() -> &'static Logger {
    GLOBAL_LOGGER.get_or_init(|| Logger::new_standalone())
}

/// Switch the global logger to embedded mode
pub fn set_embedded_mode(sender: Sender<LogMessage>) {
    get_logger().set_embedded_mode(sender);
}

/// Switch the global logger to network mode
pub fn set_network_mode(sender: watch::Sender<SchedulerNotification>) {
    get_logger().set_network_mode(sender);
}

/// Switch the global logger to dual mode (terminal + network)
pub fn set_dual_mode(sender: watch::Sender<SchedulerNotification>) {
    get_logger().set_dual_mode(sender);
}

/// Switch the global logger to standalone mode
pub fn set_standalone_mode() {
    get_logger().set_standalone_mode();
}

/// Convenience macros for logging
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().debug(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().info(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().warn(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().fatal(format!($($arg)*))
    };
}

/// Drop-in replacement for println! that goes through the logging system
#[macro_export]
macro_rules! log_println {
    () => {
        $crate::logger::get_logger().info("".to_string())
    };
    ($($arg:tt)*) => {
        $crate::logger::get_logger().info(format!($($arg)*))
    };
}

/// Drop-in replacement for eprintln! that goes through the logging system
#[macro_export]
macro_rules! log_eprintln {
    () => {
        $crate::logger::get_logger().error("".to_string())
    };
    ($($arg:tt)*) => {
        $crate::logger::get_logger().error(format!($($arg)*))
    };
}

/// Drop-in replacement for print! that goes through the logging system
#[macro_export]
macro_rules! log_print {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().info(format!($($arg)*))
    };
}

/// Drop-in replacement for eprint! that goes through the logging system
#[macro_export]
macro_rules! log_eprint {
    ($($arg:tt)*) => {
        $crate::logger::get_logger().error(format!($($arg)*))
    };
}