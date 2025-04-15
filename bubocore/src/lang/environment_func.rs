use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentFunc {
    GetTempo,
    RandomU8,
    RandomInt,
    RandomFloat,
    FrameLen(Box<Variable>, Box<Variable>)
}
pub use EnvironmentFunc::*;

use super::{evaluation_context::EvaluationContext, variable::{Variable, VariableValue}};

impl EnvironmentFunc {

    pub fn execute(&self, ctx : &EvaluationContext) -> VariableValue {
        match self {
            GetTempo => ctx.clock.session_state.tempo().into(),
            RandomU8 => (rand::random::<u8>() as i64).into(),
            RandomInt => rand::random::<i64>().into(),
            RandomFloat => rand::random::<f64>().into(),
            FrameLen(x, y) => {
                let line_i = ctx.evaluate(x).as_integer(ctx) as usize;
                let frame_i = ctx.evaluate(y).as_integer(ctx) as usize;
                ctx.lines[line_i % ctx.lines.len()].frame_len(frame_i).into()
            },
        }
    }

}
