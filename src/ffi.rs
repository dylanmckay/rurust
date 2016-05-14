use libc;

#[repr(C)]
pub struct VALUE(libc::uintptr_t);

extern "C" {
    pub fn ruby_init();
    pub fn ruby_cleanup(_: libc::c_int);
    pub fn rb_eval_string_protect(_: *const libc::c_char, _: *const libc::c_int) -> VALUE;
}
