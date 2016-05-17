use Value;

use util;
use ffi;
use libc;

struct Function {
    name: String,
    func: *mut extern fn() -> Value,
    arg_count: u8,
}

struct Constant(String, Value);

/// A module builder.
pub struct Module
{
    name: String,
    parent: Option<Value>,

    constants: Vec<Constant>,
    included_modules: Vec<Value>,
    prepended_modules: Vec<Value>,

    functions: Vec<Function>,
}

impl Module
{
    /// Creates a new module under the global scope.
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Self::new_under(name, None)
    }

    /// Creates a new module under a value (module, class, etc).
    pub fn new_under<S>(name: S, parent: Option<Value>) -> Self where S: Into<String> {
        Module {
            name: name.into(),
            parent: parent,

            constants: Vec::new(),
            included_modules: Vec::new(),
            prepended_modules: Vec::new(),

            functions: Vec::new(),
        }
    }

    /// Adds a constant to the module.
    pub fn constant<S>(mut self, name: S, value: Value) -> Self
        where S: Into<String> {
        self.constants.push(Constant(name.into(), value));
        self
    }

    /// Includes a module.
    pub fn include(mut self, module: Value) -> Self {
        self.included_modules.push(module);
        self
    }

    /// Prepends a module.
    pub fn prepend(mut self, module: Value) -> Self {
        self.prepended_modules.push(module);
        self
    }

    /// Defines a function.
    pub fn function<S>(mut self, name: S, func_addr: *mut extern fn() -> Value, arg_count: u8) -> Self
        where S: Into<String> {
        self.functions.push(Function {
            name: name.into(),
            func: func_addr,
            arg_count: arg_count,
        });
        self
    }

    pub fn build(self) -> Value {
        let name = util::c_string(&self.name);

        let value = Value::from(unsafe {
            if let Some(parent) = self.parent {
                ffi::rb_define_module_under(parent.0, name.as_ptr())
            } else {
                ffi::rb_define_module(name.as_ptr())
            }
        });

        for constant in self.constants {
            unsafe {
                ffi::rb_define_const(
                    value.0,
                    util::c_string(&constant.0).as_ptr(),
                    (constant.1).0,
                );
            }
        }

        for function in self.functions {
            unsafe {
                ffi::rb_define_module_function(
                    value.0,
                    util::c_string(&function.name).as_ptr(),
                    function.func as *mut _,
                    function.arg_count as libc::c_int,
                );
            }
        }

        for module in self.included_modules {
            unsafe { ffi::rb_include_module(value.0, module.0) };
        }

        for module in self.prepended_modules {
            unsafe { ffi::rb_prepend_module(value.0, module.0) };
        }

        value
    }
}

