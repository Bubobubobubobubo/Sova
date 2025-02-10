use crate::lang::{Program, Instruction, event::Event};
use crate::clock::TimeSpan;

static TIME_FACTOR: u64 = 100000;

#[derive(Debug)]
pub struct Prog {
    pub instructions : Vec<Inst>,
}

#[derive(Debug, Clone)]
pub enum Inst {
    EventPlayArpeggio(Vec<u64>),
    EventPlayChord(Vec<u64>),
    EventPlayNote(Vec<u64>),
}

impl Prog {
    pub fn new(i: Inst) -> Prog {
        Prog {
            instructions: vec![i]
        }
    }

    pub fn add_instruction(&mut self, i: Inst) {
        self.instructions.push(i);
    }

    pub fn as_asm(self) -> Program {
        self.instructions.iter().map(|i| i.as_asm()).flatten().collect()
    }
}

impl Inst {
    pub fn as_asm(&self) -> Vec<Instruction> {
        use self::Inst::*;
        match self {
            EventPlayArpeggio(s) => {
                let each_duration = if s.len() >= 2 { TIME_FACTOR * s[s.len() - 2] } else { 0 };
                let each_pause = if s.len() >= 3 { TIME_FACTOR * s[s.len() - 1] } else { 0 };
                let mut res = Vec::new();
                for note_pos in 0..s.len()-2 {
                    res.push(Instruction::Effect(
                        Event::Chord(vec![s[note_pos]], TimeSpan::Micros(each_duration)),
                        TimeSpan::Micros(each_pause)))
                };
                res
            }
            EventPlayChord(s) => {
                let duration = if s.len() >= 2 { TIME_FACTOR * s[s.len() - 2] } else { 0 };
                let pause = if s.len() >= 3 { TIME_FACTOR * s[s.len() - 1] } else { 0 };
                let notes = Vec::from(&s[0..s.len()-2]);
                vec![Instruction::Effect(
                    Event::Chord(notes, TimeSpan::Micros(duration)),
                    TimeSpan::Micros(pause))]
            }
            EventPlayNote(s) => {
                let note = s[0];
                let duration = if s.len() >= 2 { TIME_FACTOR * s[1] } else { 0 };
                let pause = if s.len() >= 3 { TIME_FACTOR * s[2] } else { 0 };
                vec![Instruction::Effect(
                    Event::Chord(vec![note], TimeSpan::Micros(duration)),
                    TimeSpan::Micros(pause))]
            }
        }
    }
}
