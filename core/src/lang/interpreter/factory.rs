use crate::scene::script::Script;

use super::Interpreter;

pub trait InterpreterFactory : Send + Sync {

    fn name(&self) -> &str;

    fn make_instance(&self, script : &Script) -> Result<Box<dyn Interpreter>, String>;

}