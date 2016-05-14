use ffi;
use util;

use std;
use libc;

use Value;

/// A Ruby virtual machine.
pub struct VM;

static mut VM_EXISTS: bool = false;

impl VM
{
    pub fn new() -> Result<Self, String> {
        unsafe {
            if VM_EXISTS {
                Err("can only have one Ruby VM at a time".to_owned())
            } else {
                ffi::ruby_init();
                VM_EXISTS = true;

                Ok(VM)
            }
        }
    }

    // FIXME: return result
    pub fn eval(&mut self, code: &str) -> Value {
        let mut state: libc::c_int = 0;

        let result = unsafe {
            ffi::rb_eval_string_protect(util::c_string(code).as_ptr(), &mut state)
        };

        Value::from(result)
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
