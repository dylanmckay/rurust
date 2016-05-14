use libc;

pub use self::consts::*;

#[repr(C)]
pub struct VALUE(libc::uintptr_t);

#[allow(non_upper_case_globals)]
pub mod consts {
    use super::VALUE;

    pub const Qfalse: VALUE = VALUE(0x00);
    pub const Qtrue:  VALUE = VALUE(0x02);
    pub const Qnil:   VALUE = VALUE(0x04);
    pub const Qundef: VALUE = VALUE(0x06);

    pub const IMMEDIATE_MASK: VALUE = VALUE(0x03);
    pub const FIXNUM_FLAG:    VALUE = VALUE(0x01);
    pub const FLONUM_MASK:    VALUE = VALUE(0x00);
    pub const FLONUM_FLAG:    VALUE = VALUE(0x02);
    pub const SYMBOL_FLAG:    VALUE = VALUE(0x0e);
}

extern "C" {
    pub fn ruby_init();
    pub fn ruby_cleanup(_: libc::c_int);

    pub fn rb_eval_string_protect(_: *const libc::c_char, _: *const libc::c_int) -> VALUE;

    pub fn rb_errinfo() -> VALUE;
    pub fn rb_set_errinfo(_: VALUE);
}
