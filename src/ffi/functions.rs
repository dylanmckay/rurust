use libc;
use super::*;

extern "C" {
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
