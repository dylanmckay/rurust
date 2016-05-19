use Value;

use util;
use ffi;
use libc;

struct Method {
    name: String,
    func: *mut extern fn() -> Value,
    arg_count: i8,
}

struct Constant(String, Value);

struct Alias {
    new_name: String,
    original_name: String,
}

struct Accessor {
    name: String,
    reader: bool,
    writer: bool,
}

/// A class builder.
pub struct Class
{
    name: String,
    base_class: Value,
    parent: Option<Value>,

    methods: Vec<Method>,
    singleton_methods: Vec<Method>,

    included_modules: Vec<Value>,
    prepended_modules: Vec<Value>,
    constants: Vec<Constant>,
    aliases: Vec<Alias>,
    accessors: Vec<Accessor>,
}

impl Class {
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Self::new_under(name, None)
    }

    pub fn new_under<S>(name: S, parent: Option<Value>) -> Self where S: Into<String> {
        Class {
            name: name.into(),
            base_class: ffi::rb_cObject.into(),
            parent: parent,

            methods: Vec::new(),
            singleton_methods: Vec::new(),

            included_modules: Vec::new(),
            prepended_modules: Vec::new(),
            constants: Vec::new(),
            aliases: Vec::new(),
            accessors: Vec::new(),
        }
    }

    /// Creates the class under a value.
    /// This may be a class, a module, etc.
    pub fn under(mut self, parent: Value) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Sets the base class.
    pub fn extend(mut self, base_class: Value) -> Self {
        self.base_class = base_class;
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

    /// Defines a method.
    pub fn method<S>(mut self, name: S, func_addr: *mut extern fn() -> Value, arg_count: i8) -> Self
        where S: Into<String> {
        self.methods.push(Method {
            name: name.into(),
            func: func_addr,
            arg_count: arg_count,
        });
        self
    }

    /// Defines a singleton method.
    pub fn singleton_method<S>(mut self, name: S, func_addr: *mut extern fn() -> Value, arg_count: i8) -> Self
        where S: Into<String> {
        self.singleton_methods.push(Method {
            name: name.into(),
            func: func_addr,
            arg_count: arg_count,
        });
        self
    }

    /// Defines a constant.
    pub fn constant<S>(mut self, name: S, value: Value) -> Self
        where S: Into<String> {
        self.constants.push(Constant(name.into(), value));
        self
    }

    /// Creates an alias method.
    pub fn alias<S1, S2>(mut self, new_name: S1, original_name: S2) -> Self
        where S1: Into<String>, S2: Into<String> {
        self.aliases.push(Alias {
            new_name: new_name.into(),
            original_name: original_name.into(),
        });
        self
    }

    /// Creates getter and setter methods for an ivar.
    pub fn attr_accessor<S>(self, name: S) -> Self
        where S: Into<String> {
        self.define_accessor(name, true, true)
    }

    /// Creates a getter method for an ivar.
    pub fn attr_reader<S>(self, name: S) -> Self
        where S: Into<String> {
        self.define_accessor(name, true, false)
    }

    /// Creates a setter method for an ivar.
    pub fn attr_writer<S>(self, name: S) -> Self
        where S: Into<String> {
        self.define_accessor(name, false, true)
    }

    /// Builds the class.
    pub fn build(self) -> Value {
        let name = util::c_string(&self.name);

        let value = Value::from(unsafe {
            if let Some(parent) = self.parent {
                ffi::rb_define_class_under(parent.0, name.as_ptr(), self.base_class.0)
            } else {
                ffi::rb_define_class(name.as_ptr(), self.base_class.0)
            }
        });

        for method in self.methods {
            Self::define_method(false, value, method);
        }

        for method in self.singleton_methods {
            Self::define_method(true, value, method);
        }

        for module in self.included_modules {
            unsafe { ffi::rb_include_module(value.0, module.0) };
        }

        for module in self.prepended_modules {
            unsafe { ffi::rb_prepend_module(value.0, module.0) };
        }

        for constant in self.constants {
            unsafe {
                ffi::rb_define_const(
                    value.0,
                    util::c_string(&constant.0).as_ptr(),
                    (constant.1).0,
                );
            }
        }

        for alias in self.aliases {
            unsafe {
                ffi::rb_define_alias(
                    value.0,
                    util::c_string(&alias.new_name).as_ptr(),
                    util::c_string(&alias.original_name).as_ptr(),
                );
            }
        }

        for accessor in self.accessors {
            unsafe {
                ffi::rb_define_attr(
                    value.0,
                    util::c_string(&accessor.name).as_ptr(),
                    if accessor.reader { 1 } else { 0 },
                    if accessor.writer { 1 } else { 0 },
                );
            }
        }

        value
    }

    fn define_accessor<S>(mut self, name: S, reader: bool, writer: bool) -> Self
        where S: Into<String> {
        self.accessors.push(Accessor {
            name: name.into(),
            reader: reader,
            writer: writer,
        });
        self
    }

    fn define_method(is_singleton: bool, value: Value, method: Method) {
        let f = if is_singleton {
            ffi::rb_define_module_function
        } else {
            ffi::rb_define_method
        };

        unsafe {
            f(value.0, util::c_string(&method.name).as_ptr(),
              method.func as *mut _, method.arg_count as libc::c_int);
        }
    }
}
