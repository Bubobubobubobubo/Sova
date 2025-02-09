// Doit faire traduction (Event, TimeSpan) en (ProtocolMessage, SyncTime)

use std::{collections::HashMap, rc::Rc, sync::{mpsc::{self, Receiver, Sender, TryRecvError}, Arc}, thread::JoinHandle};

use thread_priority::ThreadBuilder;

use crate::{clock::{Clock, ClockServer, SyncTime}, device_map::DeviceMap, lang::variable::VariableStore, pattern::{script::{Script, ScriptExecution}, Pattern}, protocol::TimedMessage};

pub const SCHEDULED_DRIFT : SyncTime = 30_000;

pub struct SchedulerMessage;

pub struct Scheduler {
    pub pattern : Pattern,
    pub globals : VariableStore,

    pub executions : Vec<ScriptExecution>,

    world_iface : Sender<TimedMessage>,
    devices : Arc<DeviceMap>,
    clock : Clock,

    message_source : Receiver<SchedulerMessage>,

    current_step : usize,
}

impl Scheduler {

    pub fn create(
        clock_server : Arc<ClockServer>, 
        devices : Arc<DeviceMap>, 
        world_iface : Sender<TimedMessage>
    ) -> (JoinHandle<()>, Sender<SchedulerMessage>) {
        let (tx,rx) = mpsc::channel();
        let handle = ThreadBuilder::default()
            .name("deep-BuboCore-scheduler")
            .spawn(move |_| {
                let mut sched = Scheduler::new(
                    clock_server.into(), 
                    devices, 
                    world_iface, 
                    rx
                );
                sched.do_your_thing();
            }).expect("Unable to start World");
        (handle, tx)
    }

    pub fn new(
        clock : Clock, 
        devices : Arc<DeviceMap>, 
        world_iface : Sender<TimedMessage>, 
        receiver : Receiver<SchedulerMessage>
    ) -> Scheduler {
        Scheduler {
            world_iface,
            pattern : Default::default(),
            globals : HashMap::new(),
            executions : Vec::new(),
            devices,
            clock,
            message_source : receiver,
            current_step : 0,
        }
    }

    fn step_index(&self, date : SyncTime) -> (usize, f64) {
        let track = self.pattern.current_track();
        let track_len : f64 = track.steps.iter().sum();
        let beat = self.clock.beat_at_date(date);
        let mut beat = beat % (track_len * track.speed_factor);
        let mut start_beat = 0.0f64;
        for i in 0..track.steps.len() {
            let step_len = track.steps[i] * track.speed_factor;
            if beat <= step_len {
                return (i, start_beat);
            }
            beat -= step_len;
            start_beat += track.steps[i];
        }
        return (track.steps.len() - 1, start_beat);
    }

    pub fn process_message(&mut self, msg : SchedulerMessage) {

    }

    pub fn do_your_thing(&mut self) {
        loop {
            self.clock.capture_app_state();
            match self.message_source.try_recv() {
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => (),
                Ok(msg) => self.process_message(msg),
            }
            let track = self.pattern.current_track();
            
            let date = self.theoretical_date();
            
            let (step, start_beat) = self.step_index(date);
            if step != self.current_step {
                let scheduled_date = self.clock.date_at_beat(start_beat);
                let script = Rc::clone(&track.scripts[step]);
                self.start_execution(script, scheduled_date);
                self.current_step = step;
            }
            
            self.execution_loop();
        }
    }

    #[inline]
    pub fn theoretical_date(&self) -> SyncTime {
        self.clock.micros() + SCHEDULED_DRIFT
    }

    #[inline]
    pub fn kill_all(&mut self) {
        self.executions.clear();
    }

    fn execution_loop(&mut self) {
        let scheduled_date = self.theoretical_date();
        self.executions.retain_mut(|exec| {
            if !exec.is_ready(scheduled_date) {
                return true;
            }
            if let Some((event, date)) = exec.execute_next(&mut self.globals, &self.clock) {
                let messages = self.devices.map_event(event, date, &self.clock);
                for message in messages {
                    let _ = self.world_iface.send(message);
                }
            }
            !exec.has_terminated()
        });
    }

    pub fn start_execution(&mut self, script : Rc<Script>, scheduled_date : SyncTime) {
        let execution = ScriptExecution::execute_at(script, scheduled_date);
        self.executions.push(execution);
    } 

}
