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
