use std::{collections::VecDeque, io::{Read, Write}, process::{Child, ChildStdout, Command, Stdio}};

use serde::{Deserialize, Serialize};

use crate::{clock::SyncTime, lang::{evaluation_context::EvaluationContext, event::ConcreteEvent, interpreter::{Interpreter, InterpreterFactory}, variable::{Variable, VariableValue}}, log_error, scene::script::Script};

pub const EXTERNAL_DONE_CHAR : u8 = 7;

pub struct ExternalInterpreter {
    process: Child,
    terminated: bool, // Storing termination status in a variable in order for has_terminated to only require &self and not &mut self
}

impl From<Child> for ExternalInterpreter {
    fn from(value: Child) -> Self {
        ExternalInterpreter { 
            process: value, 
            terminated: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ExternalAction {
    Var(Variable, VariableValue),
    Stack(VecDeque<VariableValue>),
    Event(ConcreteEvent),
    Delay(SyncTime),
    Terminate
}

impl ExternalInterpreter {

    fn parse_stdout(&mut self, ctx: &mut EvaluationContext, stdout: &mut ChildStdout) 
        -> (Option<ConcreteEvent>, Option<SyncTime>)
    {
        let mut buf = Vec::new();
        let mut event = None;
        let mut wait = None;
        while buf.last().map(|b| *b != EXTERNAL_DONE_CHAR).unwrap_or_default() {
            if stdout.read_to_end(&mut buf).is_err() {
                log_error!("Unable to read external interpreter output");
                return Default::default();
            }
        }
        buf.pop();
        let Ok(actions) = serde_json::from_slice::<Vec<ExternalAction>>(&buf) else {
            log_error!("Unable to parse external interpreter output");
            return Default::default();
        };
        for action in actions {
            match action {
                ExternalAction::Var(v, x) => {
                    ctx.set_var(&v, x);
                },
                ExternalAction::Stack(stack) => {
                    *ctx.stack = stack;
                },
                ExternalAction::Event(e) => event = Some(e),
                ExternalAction::Delay(d) => wait = Some(d),
                ExternalAction::Terminate => self.stop(),
            }
        }
        (event, wait)
    } 
    
}

impl Interpreter for ExternalInterpreter {

    fn execute_next(
        &mut self,
        ctx : &mut EvaluationContext
    ) -> (Option<ConcreteEvent>, Option<SyncTime>) {
        let Ok(ctx_bytes) = serde_json::to_vec(ctx) else {
            return Default::default();
        };
        if let Some(stdin) = &mut self.process.stdin {
            if stdin.write_all(&ctx_bytes).is_err() {
                log_error!("Error while sending to external STDIN");
                self.stop();
                return Default::default();
            }
        }
        if let Some(mut stdout) = self.process.stdout.take() {
            let res = self.parse_stdout(ctx, &mut stdout);
            self.process.stdout = Some(stdout);
            res
        } else {
            log_error!("Error while reading external STDOUT");
            self.stop();
            return Default::default();
        }
    }

    fn has_terminated(&self) -> bool {
        self.terminated
    }

    fn stop(&mut self) {
        let _ = self.process.kill();
        self.terminated = true;
    }

}

pub struct ExternalInterpreterFactory;

impl InterpreterFactory for ExternalInterpreterFactory {

    fn name(&self) -> &str {
        "external"
    }

    fn make_instance(&self, script : &Script) -> Result<Box<dyn Interpreter>, String> {
        let Some(command) = script.args.get("command") else {
            return Err("No command specified for external interpreter".to_owned())
        };
        let process = Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();
        
        match process {
            Ok(child) => Ok(Box::new(ExternalInterpreter::from(child))),
            Err(e) => Err(e.to_string()),
        }
    }

}