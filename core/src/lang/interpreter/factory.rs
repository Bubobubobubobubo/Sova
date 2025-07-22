use super::Interpreter;

pub trait InterpreterFactory : Send + Sync {

    fn name(&self) -> &str;

    fn make_instance(&self, content : &str) -> Box<dyn Interpreter>;

}