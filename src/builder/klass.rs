use Value;

use util;
use ffi;
use std;
use libc;

pub struct Method {
    name: String,
    func: *const u8,
    // func: *const extern fn(...) -> VALUE,
    arg_count: u8,
}

pub struct Class
{
    name: String,
    base_class: Value,
    under: Option<Value>,

    methods: Vec<Method>,
    singleton_methods: Vec<Method>,
}

impl Class {
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Class {
            name: name.into(),
            base_class: ffi::rb_cObject.into(),
            under: None,

            methods: Vec::new(),
            singleton_methods: Vec::new(),
        }
    }

    pub fn under(mut self, parent: Value) -> Self {
        self.under = Some(parent);
        self
    }

    pub fn extending(mut self, base_class: Value) -> Self {
        self.base_class = base_class;
        self
    }

    pub fn method<S>(mut self, name: S, func_addr: *const u8, arg_count: u8) -> Self
        where S: Into<String> {
        self.methods.push(Method {
            name: name.into(),
            func: func_addr,
            arg_count: arg_count,
        });
        self
    }

    pub fn singleton_method<S>(mut self, name: S, func_addr: *const u8, arg_count: u8) -> Self
        where S: Into<String> {
        self.singleton_methods.push(Method {
            name: name.into(),
            func: func_addr,
            arg_count: arg_count,
        });
        self
    }

    pub fn build(self) -> Value {
        let name = util::c_string(&self.name);

        let value = Value::from(unsafe {
            if let Some(parent) = self.under {
                ffi::rb_define_class_under(parent.0, name.as_ptr(), self.base_class.0)
            } else {
                ffi::rb_define_class(name.as_ptr(), self.base_class.0)
            }
        });

        for method in self.methods.iter() {
            unsafe {
                ffi::rb_define_method(
                    value.0,
                    util::c_string(&method.name).as_ptr(),
                    method.func as *mut _,
                    method.arg_count as libc::c_int,
                );
            }
        }

        for method in self.singleton_methods.iter() {
            unsafe {
                ffi::rb_define_module_function(
                    value.0,
                    util::c_string(&method.name).as_ptr(),
                    std::mem::transmute(method.func),
                    method.arg_count as libc::c_int,
                );
            }
        }

        value
    }
}
