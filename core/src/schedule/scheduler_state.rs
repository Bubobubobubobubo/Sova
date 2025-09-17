use crate::{
    clock::SyncTime,
    scene::Line,
    schedule::{action_timing::ActionTiming, message::SchedulerMessage},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Starting(f64),
    Playing,
}

#[derive(Debug, Clone)]
pub struct DeferredAction {
    pub action: SchedulerMessage,
    pub timing: ActionTiming,
}

impl DeferredAction {
    pub fn new(action: SchedulerMessage, timing: ActionTiming) -> Self {
        Self { action, timing }
    }

    pub fn should_apply(&self, current_beat: f64, last_beat: f64, lines: &[Line]) -> bool {
        match self.timing {
            ActionTiming::Immediate => false,
            ActionTiming::AtBeat(target) => current_beat >= target as f64,
            ActionTiming::EndOfLine(i) => {
                let len = lines[i % lines.len()].length();
                if len <= 0.0 {
                    false
                } else {
                    (last_beat % len) > (current_beat % len)
                }
            }
        }
    }
}

pub const SCHEDULED_DRIFT: SyncTime = 1_000;
