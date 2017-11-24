use context::Context;
use jcvmerrors::InterpreterError;
use exceptions;


#[derive(Debug)]
pub enum InterpreterException {
    NullPointerException,
    ArrayIndexOutOfBoundsException,
}


pub fn throw_exception(ctx: &Context,
                       except: exceptions::InterpreterException)
                       -> Result<(), InterpreterError> {
    println!("JC Exception raised !");
    panic!("WIP");
    Ok(())
}
