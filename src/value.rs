use ffi;
use util;
use std::fmt;

pub struct Value(pub ffi::VALUE);

impl Value
{
    pub fn nil() -> Self {
        Self::from(ffi::Qnil)
    }

    pub fn symbol(name: &str) -> Self {
        Self::from(unsafe { ffi::rb_id2sym(Self::intern(name)) })
    }

    pub fn string(s: &str) -> Self {
        Self::from(unsafe { ffi::rb_id2str(Self::intern(s)) })
    }

    pub fn to_sym(&self) -> Value {
        Self::from(unsafe { ffi::rb_to_symbol(self.0) })
    }

    pub fn class(&self) -> Value {
        unimplemented!();
        // Self::from(unsafe { ffi::rb_class_of(self.0) })
    }

    pub fn display_string(&self) -> String {
        self.call_no_args("to_s").as_string()
    }

    pub fn inspect_string(&self) -> String {
        self.call_no_args("inspect").as_string()
    }

    pub fn is_nil(&self)    -> bool { self.0 == ffi::Qnil }
    pub fn is_string(&self) -> bool { ffi::RB_TYPE_P(self.0, ffi::RUBY_T_STRING) }
    pub fn is_fixnum(&self) -> bool { ffi::RB_TYPE_P(self.0, ffi::RUBY_T_FIXNUM) }
    pub fn is_symbol(&self) -> bool { ffi::RB_TYPE_P(self.0, ffi::RUBY_T_SYMBOL) }
    pub fn is_float(&self)  -> bool { ffi::RB_TYPE_P(self.0, ffi::RUBY_T_FLOAT) }
    pub fn is_object(&self) -> bool { ffi::RB_TYPE_P(self.0, ffi::RUBY_T_OBJECT) }

    pub fn as_string(&self) -> String {
        if !self.is_string() { panic!("is not a string") }

        let c_str = unsafe { ffi::rb_string_value_cstr(&self.0 as *const _) };
        util::string(c_str)
    }

    pub fn call_no_args(&self, method_name: &str) -> Self {
        Self::from(unsafe { ffi::rb_funcall(self.0, Self::intern(method_name), 0) })
    }

    fn intern(s: &str) -> ffi::ID {
        unsafe { ffi::rb_intern(util::c_string(s).as_ptr()) }
    }
}

impl fmt::Display for Value
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.display_string().fmt(fmt)
    }
}

impl fmt::Debug for Value
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.display_string().fmt(fmt)
    }
}

impl From<ffi::VALUE> for Value
{
    fn from(value: ffi::VALUE) -> Value {
        Value(value)
    }
}
