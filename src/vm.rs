use ffi;
use builder;
use util;

use std;
use libc;

use Value;

/// A Ruby virtual machine.
pub struct VM;

static mut VM_EXISTS: bool = false;

#[derive(Debug)]
pub enum ErrorKind
{
    /// An internal VM error.
    VM(String),
    /// An exception was thrown.
    Exception(Value),
}

impl VM
{
    pub fn new() -> Result<Self, ErrorKind> {
        unsafe {
            if VM_EXISTS {
                Err(ErrorKind::VM("can only have one Ruby VM at a time".to_owned()))
            } else {
                ffi::ruby_init();
                VM_EXISTS = true;

                Ok(VM)
            }
        }
    }

    pub fn eval(&mut self, code: &str) -> Result<Value, ErrorKind> {
        let mut state: libc::c_int = 0;

        let result = unsafe {
            ffi::rb_eval_string_protect(util::c_string(code).as_ptr(), &mut state)
        };

        if state == 0 {
            Ok(Value::from(result))
        } else {
            Err(ErrorKind::Exception(self.consume_exception().unwrap()))
        }
    }

    pub fn class<S>(&mut self, name: S) -> builder::ClassBuilder
        where S: Into<String> {
        builder::ClassBuilder::new(name)
    }

    pub fn consume_exception(&mut self) -> Option<Value> {
        let exception = self.current_exception();

        unsafe {
            ffi::rb_set_errinfo(ffi::Qnil);
        }

        exception
    }

    // TODO: make this return None if err is nil
    pub fn current_exception(&self) -> Option<Value> {
        Some(Value::from(unsafe { ffi::rb_errinfo() }))
    }
}

impl std::ops::Drop for VM
{
    fn drop(&mut self) {
        unsafe {
            VM_EXISTS = false;
            ffi::ruby_cleanup(0);
        };
    }
}

#[cfg(test)]
describe! vm {
    before_each { let mut vm = VM::new().unwrap(); }

    it "can eval a simple assignment" {
        vm.eval("a = 1");
    }
}
