use ffi;
use util;
use libc;
use std::fmt;

#[derive(Copy,Clone)]
/// A Ruby value.
pub struct Value(pub ffi::VALUE);

impl Value
{
    /// Gets `nil`.
    pub fn nil() -> Self {
        Self::from(ffi::Qnil)
    }

    /// Creates a new symbol.
    pub fn symbol(name: &str) -> Self {
        Self::from(unsafe { ffi::rb_id2sym(Self::intern(name)) })
    }

    /// Creates a new String.
    pub fn string(s: &str) -> Self {
        Self::from(unsafe { ffi::rb_id2str(Self::intern(s)) })
    }

    /// Creates a new `Float`.
    pub fn float(v: f64) -> Self {
        Self::from(unsafe { ffi::rb_float_new(v as libc::c_double) })
    }

    /// Converts the value into a symbol.
    pub fn to_sym(&self) -> Value {
        Self::from(unsafe { ffi::rb_to_symbol(self.0) })
    }

    /// Gets the class.
    pub fn class(&self) -> Value {
        Self::from(unsafe { ffi::rb_class_of(self.0) })
    }

    /// The value of `Object#to_s`.
    pub fn display_string(&self) -> String {
        self.call_no_args("to_s").as_string().expect("Object#to_s did not return a String")
    }

    /// The value of `Object#inspect`.
    pub fn inspect_string(&self) -> String {
        self.call_no_args("inspect").as_string().expect("Object#inspect did not return a String")
    }

    /// Gets the name of the class of the object.
    pub fn class_name(&self) -> String {
        unsafe { util::string(ffi::rb_obj_classname(self.0)) }
    }

    pub fn is_nil(&self)    -> bool { self.0 == ffi::Qnil }
    pub fn is_string(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_STRING) }
    pub fn is_fixnum(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_FIXNUM) }
    pub fn is_symbol(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_SYMBOL) }
    pub fn is_float(&self)  -> bool { ffi::TYPE_P(self.0, ffi::T_FLOAT) }
    pub fn is_object(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_OBJECT) }

    /// Ruby's version of '=='
    pub fn is_equal_to(&self, other: Self) -> Self {
        Self::from(unsafe { ffi::rb_equal(self.0, other.0) })
    }

    /// Converts a Ruby `String` into a Rust `String`.
    /// Returns `None` if the value is not a Ruby `String`.
    pub fn as_string(&self) -> Option<String> {
        if self.is_string() {
            let c_str = unsafe { ffi::rb_string_value_cstr(&self.0 as *const _) };
            Some(util::string(c_str))
        } else {
            None
        }
    }

    /// Calls a method with no args.
    pub fn call_no_args(&self, method_name: &str) -> Self {
        Self::from(unsafe { ffi::rb_funcall(self.0, Self::intern(method_name), 0) })
    }

    /// Sends a message to the value.
    pub fn send(&self, method_name: &str, args: &[Self]) -> Self {
        Self::from(unsafe {
            ffi::rb_funcallv(
                self.0,
                Self::intern(method_name),
                args.len() as libc::c_int,
                args.as_ptr() as *const _,
            )
        })
    }

    /// Gets the value of an instance variable by name.
    /// Returns `nil` if it doesn't exist.
    pub fn get_ivar(&self, name: &str) -> Self {
        Self::from(unsafe { ffi::rb_iv_get(self.0, util::c_string(name).as_ptr()) })
    }

    /// Sets the value of an instance variable (or creates a new one).
    pub fn set_ivar(&self, name: &str, value: Self) -> Self {
        Self::from(unsafe { ffi::rb_iv_set(self.0, util::c_string(name).as_ptr(), value.0) })
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

