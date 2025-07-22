use crate::lang::interpreter::{Interpreter, InterpreterFactory};

pub struct BoinxInterpreter {

}

impl Interpreter for BoinxInterpreter {

    fn execute_next(
        &mut self,
        ctx : &mut crate::lang::evaluation_context::EvaluationContext
    ) -> (Option<crate::lang::event::ConcreteEvent>, Option<crate::clock::SyncTime>) {
        todo!()
    }

    fn has_terminated(&self) -> bool {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }

}

pub struct BoinxInterpreterFactory {

}

impl InterpreterFactory for BoinxInterpreterFactory {
    
    fn name(&self) -> &str {
        "boinx"
    }

    fn make_instance(&self, content : String) -> Box<dyn Interpreter> {
        todo!()
    }

}