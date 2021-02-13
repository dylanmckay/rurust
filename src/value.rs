use builder;
use ffi;
use util;
use libc;
use std::{cmp, fmt};

/// A Ruby value.
#[derive(Copy,Clone)]
#[repr(transparent)]
pub struct Value(pub ffi::VALUE);

impl Value
{
    /// Gets `nil`.
    pub fn nil() -> Self {
        Self::from(ffi::Qnil)
    }

    /// Gets a boolean value.
    pub fn boolean(b: bool) -> Self {
        if b { Value::boolean_true() } else { Value::boolean_false() }
    }

    /// Gets the boolean `true` value.
    pub fn boolean_true() -> Self {
        Self::from(ffi::Qtrue)
    }

    /// Gets the boolean `false` value.
    pub fn boolean_false() -> Self {
        Self::from(ffi::Qfalse)
    }

    /// Creates a new symbol.
    pub fn symbol<S>(name: S) -> Self where S: AsRef<str> {
        Self::from(unsafe { ffi::rb_id2sym(Self::intern(name.as_ref())) })
    }

    /// Creates a new String.
    pub fn string<S>(s: S) -> Self where S: AsRef<str> {
        Self::from(unsafe { ffi::rb_id2str(Self::intern(s.as_ref())) })
    }

    /// Creates a new `Integer`.
    pub fn integer<I>(v: I) -> Self where I: Into<i64> {
        // FIXME: this can overflow and panic
        Self::from(ffi::INT2FIX(v.into() as usize))
    }

    /// Creates a new `Float`.
    pub fn float<F>(v: F) -> Self where F: Into<f64> {
        Self::from(unsafe { ffi::rb_float_new(v.into()) })
    }

    /// Converts the value into a symbol.
    pub fn to_sym(&self) -> Value {
        Self::from(unsafe { ffi::rb_to_symbol(self.0) })
    }

    /// Converts the value to a 64-bit signed integer.
    pub fn to_i64(&self) -> i64 {
        unsafe { ffi::rb_num2long(self.0) as i64 }
    }

    /// Converts the value to a 64-bit unsigned integer.
    pub fn to_u64(&self) -> u64 {
        unsafe { ffi::rb_num2ulong(self.0) as u64 }
    }

    /// Converts the value into a 64-bit float.
    pub fn to_f64(&self) -> f64 {
        unsafe { ffi::rb_num2dbl(self.0) as f64 }
    }

    /// Gets the class.
    pub fn class(&self) -> Value {
        Self::from(unsafe { ffi::rb_class_of(self.0) })
    }

    /// Creates a nested class.
    pub fn nested_class<S>(self, name: S) -> builder::Class
        where S: Into<String> {
        builder::Class::new_under(name, Some(self))
    }

    /// Creates a nested module.
    pub fn nested_module<S>(self, name: S) -> builder::Module
        where S: Into<String> {
        builder::Module::new_under(name, Some(self))
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

    /// Checks if the value is `nil`.
    pub fn is_nil(&self)   -> bool { self.0 == ffi::Qnil }
    /// Checks if the value is `true`.
    pub fn is_true(&self)  -> bool { self.0 == ffi::Qtrue }
    /// Checks if the value is `false.`
    pub fn is_false(&self) -> bool { self.0 == ffi::Qfalse }

    /// Checks if the value is a `String` type.
    pub fn is_string(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_STRING) }
    /// Checks if the value is a regex.
    pub fn is_regex(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_REGEXP) }
    /// Checks if the value is an `Integer` type.
    pub fn is_integer(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_FIXNUM) }
    /// Checks if the value is a complex number.
    pub fn is_complex_number(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_COMPLEX) }
    /// Checks if the value is a rational number.
    pub fn is_rational(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_RATIONAL) }
    /// Checks if the value is a symbol.
    pub fn is_symbol(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_SYMBOL) }
    /// Checks if the value is a float.
    pub fn is_float(&self)  -> bool { ffi::TYPE_P(self.0, ffi::T_FLOAT) }
    /// Checks if the value is an array.
    pub fn is_array(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_ARRAY) }
    /// Checks if the value is a hash.
    pub fn is_hash(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_HASH) }
    /// Checks if the value is an object.
    pub fn is_object(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_OBJECT) }
    /// Checks if the value is a class.
    pub fn is_class(&self)  -> bool { ffi::TYPE_P(self.0, ffi::T_CLASS) }
    /// Checks if the value is a `Struct`.
    pub fn is_struct(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_STRUCT) }
    /// Checks if the value is a module.
    pub fn is_module(&self) -> bool { ffi::TYPE_P(self.0, ffi::T_MODULE) }

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

impl cmp::PartialEq for Value
{
    fn eq(&self, rhs: &Value) -> bool {
        self.is_equal_to(*rhs).is_true()
    }
}

impl cmp::Eq for Value { }

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

#[cfg(test)]
mod test {
    use super::Value;

    #[test]
    fn can_create_booleans() {
        assert!(Value::boolean_true().is_true());
        assert!(Value::boolean_false().is_false());
        assert!(Value::boolean(true).is_true());
        assert!(Value::boolean(false).is_false());
    }

    #[test]
    fn can_create_integers() {
        assert_eq!(50, Value::integer(50).to_u64());
        assert_eq!(0xdeadbe, Value::integer(0xdeadbe).to_u64());
    }
}

