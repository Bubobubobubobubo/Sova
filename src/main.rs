use std::time::{SystemTime, UNIX_EPOCH};

use clock::TimeSpan;
use lang::{Event, Instruction, Program};
use protocol::{log::{LogMessage, Severity}, ProtocolMessage};
use world::World;
use dummytranslator::translate;

pub mod schedule;
pub mod clock;
pub mod io;
pub mod world;
pub mod protocol;
pub mod lang;
pub mod dummytranslator;
pub mod dummyast;
pub mod pattern;

fn main() {

    let (handle, message_sender) = World::create();

    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backward");
    let now = since_epoch.as_micros() as u64;

    let sender2 = message_sender.clone();

    for i in 0..10 {
        let log0 = LogMessage::new(Severity::Debug, "Hello world !".to_owned());
        let log0 = ProtocolMessage::LOG(log0).timed(now + i * 1000 * 1000 * (i % 2));
        sender2.send(log0).unwrap();
    }

    // This is a test program for the scheduler
    let crashtest_program: Program = vec![
        Instruction::Effect(
            Event::Note(60, TimeSpan::Micros(1)),
            TimeSpan::Micros(2)
        ),
        Instruction::Effect(
            Event::Exit,
            TimeSpan::Micros(4)
        )
    ];

    // This is a test program obtained from a script
    let crashtest_parsed_program: Program = translate("N 5 2 1 C 3 7 100 4 5");

    handle.join().expect("Thread error");
}
