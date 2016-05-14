use super::*;
use std::mem;
use libc;

/// Gets the class of a `VALUE`.
/// This is actually defined in the Ruby library, but it is inline.
/// This is a port of it.
pub unsafe fn rb_class_of(obj: VALUE) -> VALUE {
    if IMMEDIATE_P(obj) {
        if FIXNUM_P(obj) { return rb_cFixnum; }
        if FLONUM_P(obj) { return rb_cFloat; }
        if obj == Qtrue { return rb_cTrueClass; }
        if STATIC_SYM_P(obj) { return rb_cSymbol; }
    } else if !RTEST(obj) {
        if obj == Qnil   { return rb_cNilClass; }
        if obj == Qfalse { return rb_cFalseClass; }
    }
    return (*RBasic::from_pointer(obj)).klass;
}

pub fn TYPE_P(obj: VALUE, ty: value_type) -> bool {
    match ty {
        value_type::T_FIXNUM => FIXNUM_P(obj),
        value_type::T_TRUE => obj == Qtrue,
        value_type::T_FALSE => obj == Qfalse,
        value_type::T_NIL => obj == Qnil,
        value_type::T_UNDEF => obj == Qundef,
        value_type::T_SYMBOL => SYMBOL_P(obj),
        value_type::T_FLOAT => FLOAT_TYPE_P(obj),
        _ => !SPECIAL_CONST_P(obj) && BUILTIN_TYPE(obj) == ty
    }
}

pub fn FLOAT_TYPE_P(obj: VALUE) -> bool {
    FLONUM_P(obj) || (!SPECIAL_CONST_P(obj) &&
                      BUILTIN_TYPE(obj) == T_FLOAT)
}

pub fn BUILTIN_TYPE(x: VALUE) -> value_type {
    unsafe {
        let basic: *const RBasic = mem::transmute(x);
        let masked = (*basic).flags.0 & (T_MASK as libc::size_t);
        mem::transmute(masked as u32)
    }
}

#[cfg(not(mri_use_flonum))]
pub fn FLONUM_P(_: VALUE) -> bool { false }

#[cfg(mri_use_flonum)]
pub fn FLONUM_P(x: VALUE) -> bool {
    x & FLONUM_MASK == FLONUM_FLAG.0
}

pub fn FIXNUM_P(f: VALUE) -> bool {
    (f & FIXNUM_FLAG) != 0
}

pub fn DYNAMIC_SYM_P(x: VALUE) -> bool {
    !SPECIAL_CONST_P(x) && BUILTIN_TYPE(x) == T_SYMBOL
}

pub fn STATIC_SYM_P(x: VALUE) -> bool {
    (x.0 & !((!0 as libc::uintptr_t) << SPECIAL_SHIFT)) == SYMBOL_FLAG.0
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
