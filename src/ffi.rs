use libc;

pub use self::consts::*;
pub use self::ruby_value_type::*;
pub use self::ty::*;
use std;

#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct VALUE(libc::uintptr_t);

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct ID(libc::uintptr_t);

#[repr(C)]
pub struct RBasic {
    flags: VALUE,
    klass: VALUE,
}

#[allow(non_upper_case_globals)]
pub mod consts {
    use super::VALUE;

    pub const Qfalse: VALUE = VALUE(0x00);

    #[cfg(mri_use_flonum)]
    pub const Qtrue:  VALUE = VALUE(0x14);
    #[cfg(not(mri_use_flonum))]
    pub const Qtrue:  VALUE = VALUE(0x02);

    #[cfg(mri_use_flonum)]
    pub const Qnil:   VALUE = VALUE(0x08);
    #[cfg(not(mri_use_flonum))]
    pub const Qnil:   VALUE = VALUE(0x04);

    #[cfg(mri_use_flonum)]
    pub const Qundef: VALUE = VALUE(0x34);
    #[cfg(not(mri_use_flonum))]
    pub const Qundef: VALUE = VALUE(0x06);

    #[cfg(mri_use_flonum)]
    pub const IMMEDIATE_MASK: VALUE = VALUE(0x07);
    #[cfg(not(mri_use_flonum))]
    pub const IMMEDIATE_MASK: VALUE = VALUE(0x03);

    pub const FIXNUM_FLAG:    VALUE = VALUE(0x01);

    #[cfg(mri_use_flonum)]
    pub const FLONUM_MASK:    VALUE = VALUE(0x03);
    #[cfg(not(mri_use_flonum))]
    pub const FLONUM_MASK:    VALUE = VALUE(0x00);

    pub const FLONUM_FLAG:    VALUE = VALUE(0x02);

    #[cfg(mri_use_flonum)]
    pub const SYMBOL_FLAG:    VALUE = VALUE(0x0c);
    #[cfg(not(mri_use_flonum))]
    pub const SYMBOL_FLAG:    VALUE = VALUE(0x0e);
}

#[repr(i32)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum ruby_value_type {
    RUBY_T_NONE     = 0x00,
    RUBY_T_OBJECT   = 0x01,
    RUBY_T_CLASS    = 0x02,
    RUBY_T_MODULE   = 0x03,
    RUBY_T_FLOAT    = 0x04,
    RUBY_T_STRING   = 0x05,
    RUBY_T_REGEXP   = 0x06,
    RUBY_T_ARRAY    = 0x07,
    RUBY_T_HASH     = 0x08,
    RUBY_T_STRUCT   = 0x09,
    RUBY_T_BIGNUM   = 0x0a,
    RUBY_T_FILE     = 0x0b,
    RUBY_T_DATA     = 0x0c,
    RUBY_T_MATCH    = 0x0d,
    RUBY_T_COMPLEX  = 0x0e,
    RUBY_T_RATIONAL = 0x0f,
    RUBY_T_NIL      = 0x11,
    RUBY_T_TRUE     = 0x12,
    RUBY_T_FALSE    = 0x13,
    RUBY_T_SYMBOL   = 0x14,
    RUBY_T_FIXNUM   = 0x15,
    RUBY_T_UNDEF    = 0x1b,
    RUBY_T_NODE     = 0x1c,
    RUBY_T_ICLASS   = 0x1d,
    RUBY_T_ZOMBIE   = 0x1e,
    RUBY_T_MASK     = 0x1f
}

extern "C" {
    pub static rb_cFixnum: VALUE;

    pub fn ruby_init();
    pub fn ruby_cleanup(_: libc::c_int);

    pub fn rb_eval_string_protect(_: *const libc::c_char, _: *const libc::c_int) -> VALUE;

    pub fn rb_errinfo() -> VALUE;
    pub fn rb_set_errinfo(_: VALUE);

    pub fn rb_intern(_: *const libc::c_char) -> ID;

    pub fn rb_id2sym(_: ID) -> VALUE;
    pub fn rb_id2str(_: ID) -> VALUE;

    pub fn rb_to_symbol(_: VALUE) -> VALUE;

    pub fn rb_funcall(_: VALUE, _: ID, _: libc::c_int, ...) -> VALUE;
    pub fn rb_string_value_cstr(_: *const VALUE) -> *const libc::c_char;
}

// Type related functions.
// Taken from https://github.com/jdm/ruby-mri-rs/
#[allow(non_snake_case)]
pub mod ty
{
    use super::*;
    use std::mem;
    use libc;

    /// Gets the class of a `VALUE`.
    /// This is actually defined in the Ruby library, but it is inline.
    /// This is a port of it.
    // pub fn rb_class_of(obj: VALUE) -> VALUE {
    //     if RB_IMMEDIATE_P(obj) {
    //         if RB_FIXNUM_P(obj) { return 
    //     }
    // }

    // static inline VALUE rb_class_of(VALUE obj)
    // {
    //     if (RB_IMMEDIATE_P(obj)) {
    //         if (RB_FIXNUM_P(obj)) return rb_cFixnum;
    //         if (RB_FLONUM_P(obj)) return rb_cFloat;
    //         if (obj == RUBY_Qtrue)  return rb_cTrueClass;
    //         if (RB_STATIC_SYM_P(obj)) return rb_cSymbol;
    //     }
    //     else if (!RTEST(obj)) {
    //         if (obj == RUBY_Qnil)   return rb_cNilClass;
    //         if (obj == RUBY_Qfalse) return rb_cFalseClass;
    //     }
    //     return RBASIC(obj)->klass;
    // }

    pub fn RB_TYPE_P(obj: VALUE, ty: ruby_value_type) -> bool {
        match ty {
            ruby_value_type::RUBY_T_FIXNUM => FIXNUM_P(obj),
            ruby_value_type::RUBY_T_TRUE => obj == Qtrue,
            ruby_value_type::RUBY_T_FALSE => obj == Qfalse,
            ruby_value_type::RUBY_T_NIL => obj == Qnil,
            ruby_value_type::RUBY_T_UNDEF => obj == Qundef,
            ruby_value_type::RUBY_T_SYMBOL => SYMBOL_P(obj),
            ruby_value_type::RUBY_T_FLOAT => RB_FLOAT_TYPE_P(obj),
            _ => !SPECIAL_CONST_P(obj) && BUILTIN_TYPE(obj) == ty
        }
    }

    pub fn RB_FLOAT_TYPE_P(obj: VALUE) -> bool {
        FLONUM_P(obj) || (!SPECIAL_CONST_P(obj) &&
                          BUILTIN_TYPE(obj) == RUBY_T_FLOAT)
    }

    pub fn BUILTIN_TYPE(x: VALUE) -> ruby_value_type {
        unsafe {
            let basic: *const RBasic = mem::transmute(x);
            let masked = (*basic).flags.0 & (RUBY_T_MASK as libc::size_t);
            mem::transmute(masked as u32)
        }
    }

    pub fn FLONUM_P(_x: VALUE) -> bool {
        false
        //x & FLONUM_MASK == FLONUM_FLAG
    }

    pub fn FIXNUM_P(f: VALUE) -> bool {
        (f & FIXNUM_FLAG) != 0
    }

    pub fn DYNAMIC_SYM_P(_x: VALUE) -> bool {
        false
    }

    pub fn STATIC_SYM_P(_x: VALUE) -> bool {
        false
    }

    pub fn SYMBOL_P(x: VALUE) -> bool {
        STATIC_SYM_P(x) || DYNAMIC_SYM_P(x)
    }

    pub fn SPECIAL_CONST_P(x: VALUE) -> bool {
        IMMEDIATE_P(x) || !RTEST(x)
    }

    pub fn IMMEDIATE_P(x: VALUE) -> bool {
        (x & IMMEDIATE_MASK) != 0
    }

    pub fn RTEST(v: VALUE) -> bool {
        (v.0 & !Qnil.0) != 0
    }

    pub fn NIL_P(v: VALUE) -> bool {
        v == Qnil
    }
}

impl std::fmt::Debug for VALUE {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "0x{:x}", self.0)
    }
}

impl std::ops::BitAnd for VALUE {
    type Output = libc::uintptr_t;

    fn bitand(self, rhs: Self) -> libc::uintptr_t {
        self.0 & rhs.0
    }
}
