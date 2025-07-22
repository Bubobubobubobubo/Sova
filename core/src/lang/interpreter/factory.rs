use super::Interpreter;

pub trait InterpreterFactory {

    fn name(&self) -> &str;

    fn make_instance(&self, content : String) -> Box<dyn Interpreter>;

}