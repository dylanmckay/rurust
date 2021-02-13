use crate::{builder, ffi, util, Value};

use std;
use std::fmt;
use libc;

use std::sync::Mutex;

/// A Ruby virtual machine.
pub struct VM;

/// We only want to be able to have one `VM` at a time.
static mut VM_EXISTS: bool = false;

lazy_static! {
    static ref ACTIVE_VM: Mutex<VM> = Mutex::new(VM::new().expect("failed to create Ruby VM"));
}

// TODO:
// Implement hooked variables (rb_define_hooked_variable)
//   Allows a callback to get/set variable value

#[derive(PartialEq)]
/// A Ruby error
pub enum ErrorKind
{
    /// An internal VM error.
    VM(String),
    /// An exception was thrown.
    Exception(Value),
}

impl VM
{
    /// Gets the active VM.
    pub fn get() -> &'static Mutex<VM> {
        &ACTIVE_VM
    }

    /// Creates a new Ruby VM.
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

    /// Evaluates a line of code.
    pub fn eval(&mut self, code: &str) -> Result<Value, ErrorKind> {
        self.eval_advanced(code, false)
    }

    /// Evaluates a line of code in a sandbox.
    ///
    /// Any variables defined will not be saved.
    pub fn eval_sandbox(&mut self, code: &str) -> Result<Value, ErrorKind> {
        self.eval_advanced(code, true)
    }

    /// `require`s a file.
    pub fn require(&self, file_name: &str) -> Value {
        Value::from(unsafe { ffi::rb_require(util::c_string(file_name).as_ptr()) })
    }

    /// Creates a new class.
    pub fn class<S>(&mut self, name: S) -> builder::Class
        where S: Into<String> {
        builder::Class::new(name)
    }

    /// Creates a new module.
    pub fn module<S>(&mut self, name: S) -> builder::Module
        where S: Into<String> {
        builder::Module::new(name)
    }

    /// Sets the value of a global variable or creates a new one.
    pub fn set_global(&self, name: &str, value: Value) -> Value {
        Value::from(unsafe { ffi::rb_gv_set(util::c_string(name).as_ptr(), value.0) })
    }

    /// Gets the value of a global variable.
    pub fn get_global(&self, name: &str) -> Value {
        Value::from(unsafe { ffi::rb_gv_get(util::c_string(name).as_ptr()) })
    }

    /// Sets a global constant.
    pub fn set_global_const(&self, name: &str, value: Value) {
        unsafe { ffi::rb_define_global_const(util::c_string(name).as_ptr(), value.0) }
    }

    /// Defines a global function.
    pub fn define_global_function(&self, name: &str, f: *mut extern fn() -> Value, arg_count: u8) -> Value {
        unsafe {
            Value::from(ffi::rb_define_global_function(
                util::c_string(name).as_ptr(),
                f as *mut _,
                arg_count as libc::c_int,
            ))
        }
    }

    /// Gets the current receiver (can be `nil`).
    pub fn current_receiver(&self) -> Value {
        unsafe { Value::from(ffi::rb_current_receiver()) }
    }

    /// Raises an object and a message.
    pub fn raise(&self, value: Value, message: &str) -> ! {
        unsafe { ffi::rb_raise(value.0, util::c_string(message).as_ptr()) }
    }

    /// Raises a fatal error.
    pub fn fatal(&self, message: &str) -> ! {
        unsafe { ffi::rb_fatal(util::c_string(message).as_ptr()) }
    }

    /// Raises a bug.
    pub fn bug(&self, message: &str) -> ! {
        unsafe { ffi::rb_bug(util::c_string(message).as_ptr()) }
    }

    /// Logs a Ruby warning.
    pub fn warning(&self, message: &str) {
        unsafe { ffi::rb_warning(util::c_string(message).as_ptr()) }
    }

    /// Prints Ruby version info to stdout.
    pub fn show_ruby_version(&self) { unsafe { ffi::ruby_show_version() } }
    /// Prints Ruby copyright info to stdout.
    pub fn show_ruby_copyright(&self) { unsafe { ffi::ruby_show_copyright() } }

    /// Sets the script name.
    /// Essentially the same as `$0 = name`.
    pub fn set_script_name(&self, name: &str) {
        unsafe { ffi::ruby_script(util::c_string(name).as_ptr()) }
    }

    /// Gets the currently raised exception and clears it.
    pub fn consume_exception(&mut self) -> Value {
        let exception = self.current_exception();
        unsafe { ffi::rb_set_errinfo(ffi::Qnil) };
        exception
    }

    /// Gets the currently raised exception exception.
    ///
    /// Can be `nil`.
    pub fn current_exception(&self) -> Value {
        Value::from(unsafe { ffi::rb_errinfo() })
    }

    fn eval_advanced(&mut self, code: &str, sandbox: bool) -> Result<Value, ErrorKind> {
        let mut state: libc::c_int = 0;

        let eval_fn = if sandbox {
            ffi::rb_eval_string_protect
        } else {
            ffi::rb_eval_string_wrap
        };

        let result = unsafe { eval_fn(util::c_string(code).as_ptr(), &mut state) };

        if state == 0 {
            Ok(Value::from(result))
        } else {
            Err(ErrorKind::Exception(self.consume_exception()))
        }
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

impl fmt::Debug for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::VM(ref msg) => write!(fmt, "virtual machine error: {}", msg),
            ErrorKind::Exception(e) => write!(fmt, "{}: {:?}", e.class_name(), e),
        }
    }
}

