use bytecodes::bytecode;
use context::Context;
use stack::StackEntry;
use jcvmerrors::InterpreterError;
use constants;
use exceptions;
use traits::{BufferAccessor, HasType};

// macro allowing to simplify null reference check
#[macro_export]
macro_rules! check_null_reference {
    ($variable:ident, $ctx:ident) => (
        if !$variable.is_of_type(constants::PrimitiveType::REFERENCE)
            || $variable.value == constants::NULL_HANDLE {
                exceptions::throw_exception($ctx,
                     exceptions::InterpreterException::NullPointerException);
        }
    )
}

///
/// Manages aaload, baload, saload, iaload
///
pub fn xaload(execution_context: &mut Context, type_: constants::PrimitiveType) {
    let arrayref = execution_context
        .operand_stack
        .pop_check_type(constants::PrimitiveType::REFERENCE)
        .unwrap();
    let index = execution_context
        .operand_stack
        .pop_check_type(constants::PrimitiveType::SHORT)
        .unwrap();

    check_null_reference!(arrayref, execution_context);

    let associated_reference = execution_context
        .object_manager
        .get_object(arrayref.value as usize);

    if let Ok(e) = associated_reference {
        // consistency check to make sure it is an array
        assert!(e.is_array());

        // in case of arrays, the primitive type represents the type of its elements
        assert!(e.is_of_type(type_));

        match type_ {
            constants::PrimitiveType::SHORT | constants::PrimitiveType::REFERENCE => {
                let size_one_entry = constants::REFERENCE_SIZE;
                match e.read_s((index.value as usize) * size_one_entry) {
                    // REFERENCE_SIZE == SHORT_SIZE here
                    Ok(res) => {
                        execution_context
                            .operand_stack
                            .push(StackEntry::from_values(res, type_));
                    }
                    Err(e) => {
                        exceptions::throw_exception_from_interpretererror(execution_context, e);
                    }
                }
            }

            constants::PrimitiveType::BYTE => {
                let size_one_entry = constants::BYTE_SIZE;
                match e.read_b((index.value as usize) * size_one_entry) {
                    // retrieve v(alue of the reference of the array
                    Ok(res) => {
                        execution_context.operand_stack.bpush(res);
                    }
                    Err(e) => {
                        exceptions::throw_exception_from_interpretererror(execution_context, e)
                    }
                }
            }

            constants::PrimitiveType::INTEGER => {
                let size_one_entry = constants::INTEGER_SIZE;
                match e.read_i((index.value as usize) * size_one_entry) {
                    // retrieve value of the reference of the array
                    Ok(res) => {
                        execution_context.operand_stack.ipush(res);
                    }
                    Err(e) => {
                        exceptions::throw_exception_from_interpretererror(execution_context, e)
                    }
                }
            }

            constants::PrimitiveType::UNKNOWN => {
                panic!("Unknown type !");
            }
        }
    } else {
        exceptions::throw_exception(execution_context, associated_reference.err().unwrap());
    }
}
