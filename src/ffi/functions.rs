use libc;
use super::*;

/// A function with vardic arguments.
///
/// Rust has the restriction that any vardic function must have at
/// least one named argument.
pub type VarargsFn<R> = extern fn(_: u32, ...) -> R;

extern "C" {
    pub fn ruby_init();
    pub fn ruby_setup() -> libc::c_int;
    pub fn ruby_cleanup(_: libc::c_int);

    pub fn rb_eval_string_protect(_: *const libc::c_char, _: *const libc::c_int) -> VALUE;

    pub fn rb_errinfo() -> VALUE;
    pub fn rb_set_errinfo(_: VALUE);

    pub fn rb_intern(_: *const libc::c_char) -> ID;

    pub fn rb_id2sym(_: ID) -> VALUE;
    pub fn rb_id2str(_: ID) -> VALUE;

    pub fn rb_to_symbol(_: VALUE) -> VALUE;

    pub fn rb_funcall(_: VALUE, _: ID, _: libc::c_int, ...) -> VALUE;
    pub fn rb_funcallv(_: VALUE, _: ID, _: libc::c_int, _: *const VALUE) -> VALUE;
    pub fn rb_funcallv_public(_: VALUE, _: ID, _: libc::c_int, _: *const VALUE) -> VALUE;
    pub fn rb_funcall_passing_block(_: VALUE, _: ID, _: libc::c_int, _: *const VALUE) -> VALUE;
    pub fn rb_funcall_with_block(_: VALUE, _: ID, _: libc::c_int, _: *const VALUE, _: VALUE) -> VALUE;
    pub fn rb_string_value_cstr(_: *const VALUE) -> *const libc::c_char;

    pub fn rb_define_class(_: *const libc::c_char, _: VALUE) -> VALUE;
    pub fn rb_define_module(_: *const libc::c_char) -> VALUE;
    pub fn rb_define_class_under(_: VALUE, _: *const libc::c_char, _: VALUE) -> VALUE;
    pub fn rb_define_module_under(_: VALUE, _: *const libc::c_char) -> VALUE;

    pub fn rb_include_module(_: VALUE, _: VALUE) -> VALUE;
    pub fn rb_extend_object(_: VALUE, _: VALUE) -> VALUE;
    pub fn rb_prepend_module(_: VALUE, _: VALUE) -> VALUE;

    pub fn rb_define_variable(_: *const libc::c_char, _: *const VALUE) -> VALUE;
    pub fn rb_define_readonly_variable(_: *const libc::c_char, _: *const VALUE) -> VALUE;
    pub fn rb_define_const(_: VALUE, _: *const libc::c_char, _: VALUE) -> VALUE;

    pub fn rb_define_method(_: VALUE, _: *const libc::c_char, _: *const VarargsFn<VALUE>, _: libc::c_int) -> VALUE;
    pub fn rb_define_module_function(_: VALUE, _: *const libc::c_char, _: VarargsFn<VALUE> , _: libc::c_int) -> VALUE;
    pub fn rb_define_global_function(_: *const libc::c_char, _: VarargsFn<VALUE>, _: libc::c_int) -> VALUE;

    pub fn rb_undef_method(_: VALUE, _: *const libc::c_char) -> VALUE;
    pub fn rb_define_alias(_: VALUE, _: *const libc::c_char, _: *const libc::c_char) -> VALUE;
    pub fn rb_define_attr(_: VALUE, _: *const libc::c_char, _: libc::c_int, _: libc::c_int) -> VALUE;

    pub fn rb_global_variable(_: *mut VALUE) -> VALUE;
    pub fn rb_gc_register_mark_object(_: *mut VALUE) -> VALUE;
    pub fn rb_gc_register_address(_: *mut VALUE) -> VALUE;
    pub fn rb_gc_unregister_address(_: *mut VALUE) -> VALUE;

    pub fn rb_scan_args(_: libc::c_int, _: *const VALUE, _: *const libc::c_char, ...) -> libc::c_int;
    pub fn rb_call_super(_: libc::c_int, _: *const VALUE) -> VALUE;
    pub fn rb_current_receiver() -> VALUE;
    pub fn rb_get_kwargs(keyword_hash: VALUE, table: *const ID, required: libc::c_int, optional: libc::c_int, _: *const VALUE) -> libc::c_int;
    pub fn rb_extract_keywords(orighash: *mut VALUE) -> VALUE;

    pub fn rb_gv_set(_: *const libc::c_char, _: VALUE) -> VALUE;
    pub fn rb_gv_get(_: *const libc::c_char) -> VALUE;
    pub fn rb_iv_get(_: VALUE, _: *const libc::c_char) -> VALUE;
    pub fn rb_iv_set(_: VALUE, _: *const libc::c_char, _: VALUE) -> VALUE;

    pub fn rb_equal(_: VALUE, _: VALUE) -> VALUE;

    pub fn rb_raise(_: VALUE, _: *const libc::c_char, ...) -> !;
    pub fn rb_fatal(_: *const libc::c_char, ...) -> !;
    pub fn rb_bug(_: *const libc::c_char, ...) -> !;
    pub fn rb_bug_errno(_: *const libc::c_char, _: libc::c_int) -> !;
    pub fn rb_sys_fail(_: *const libc::c_char) -> !;
    pub fn rb_sys_fail_str(_: VALUE) -> !;
    pub fn rb_mod_sys_fail(_: VALUE, _: *const libc::c_char) -> !;
    pub fn rb_mod_sys_fail_str(_: VALUE, _: VALUE) -> !;
    pub fn rb_readwrite_sys_fail(rb_io_wait_readwrite: libc::c_int, _: *const libc::c_char) -> !;
    pub fn rb_iter_break() -> !;
    pub fn rb_iter_break_value(_: VALUE) -> !;
    pub fn rb_exit(_: libc::c_int) -> !;
    pub fn rb_notimplement() -> !;
    pub fn rb_syserr_new(_: libc::c_int, _: *const libc::c_char) -> VALUE;
    pub fn rb_syserr_new_str(n: libc::c_int, arg: VALUE) -> VALUE;
    pub fn rb_syserr_fail(_: libc::c_int, _: *const libc::c_char) -> !;
    pub fn rb_syserr_fail_str(_: libc::c_int, _: VALUE) -> !;
    pub fn rb_mod_syserr_fail(_: VALUE, _: libc::c_int, _: *const libc::c_char) -> !;
    pub fn rb_mod_syserr_fail_str(_: VALUE, _: libc::c_int, _: VALUE) -> !;
    pub fn rb_readwrite_syserr_fail(rb_io_wait_readwrite: libc::c_int, _: libc::c_int, _: *const libc::c_char) -> !;

    pub fn rb_warning(format: *const libc::c_char, ...);
    pub fn rb_compile_warning(format: *const libc::c_char, _: libc::c_int, _: *const libc::c_char, ...);
    pub fn rb_sys_warning(format: *const libc::c_char, ...);
    pub fn rb_warn(format: *const libc::c_char, ...);
    pub fn rb_compile_warn(_: *const libc::c_char, _: libc::c_int, _: *const libc::c_char, ...);

    pub fn rb_each(_: VALUE) -> VALUE;
    pub fn rb_yield(_: VALUE) -> VALUE;
    pub fn rb_yield_values(n: libc::c_int, ...) -> VALUE;
    pub fn rb_yield_values2(n: libc::c_int, argv: *const VALUE) -> VALUE;
    pub fn rb_yield_splat(_: VALUE) -> VALUE;
    pub fn rb_yield_block(_: VALUE, _: VALUE, _: libc::c_int, _: *const VALUE, _: VALUE) -> VALUE;
    pub fn rb_block_given_pvoid() -> libc::c_int;
    pub fn rb_need_block();
    pub fn rb_iterate(_: *const extern fn(VALUE) -> VALUE, _: VALUE, _: *const extern fn(VALUE) -> VALUE, _: VALUE) -> VALUE;
    // pub fn rb_rescue(VALUE(*)(ANYARGS),VALUE,VALUE(*)(ANYARGS),VALUE) -> VALUE;
    // pub fn rb_rescue2(VALUE(*)(ANYARGS),VALUE,VALUE(*)(ANYARGS),VALUE,...) -> VALUE;
    // pub fn rb_ensure(VALUE(*)(ANYARGS),VALUE,VALUE(*)(ANYARGS),VALUE) -> VALUE;
    // pub fn rb_catch(const char*,VALUE(*)(ANYARGS),VALUE) -> VALUE;
    // pub fn rb_catch_obj(VALUE,VALUE(*)(ANYARGS),VALUE) -> VALUE;
    pub fn rb_throw(_: *const libc::c_char, _: VALUE) -> !;
    pub fn rb_throw_obj(_: VALUE, _: VALUE) -> !;

    pub fn rb_require(_: *const libc::c_char) -> VALUE;

    pub fn ruby_sysinit(argc: *mut libc::c_int, _: *mut *mut *mut libc::c_char);
    pub fn ruby_options(argc: libc::c_int, argv: *mut *mut libc::c_char) -> *mut libc::c_void;
    pub fn ruby_executable_node(n: *mut libc::c_void, status: *mut libc::c_int) -> libc::c_int;
    pub fn ruby_run_node(n: *mut libc::c_void) -> libc::c_int;
    pub fn ruby_show_version();
    pub fn ruby_show_copyright();

    pub fn ruby_finalize();
    pub fn ruby_stop(_: libc::c_int) -> !;

    pub fn ruby_set_stack_size(_: libc::size_t);
    pub fn ruby_stack_check() -> libc::c_int;
    pub fn ruby_stack_length(_: *mut *mut VALUE) -> libc::size_t;

    pub fn ruby_exec_node(n: *mut libc::c_void) -> libc::c_int;

    pub fn ruby_script(name: *const libc::c_char);
    pub fn ruby_set_script_name(name: VALUE);

    pub fn ruby_prog_init();
    pub fn ruby_set_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    pub fn ruby_process_options(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut libc::c_void;
    pub fn ruby_init_loadpath();
    pub fn ruby_incpush(_: *const libc::c_char);
    pub fn ruby_sig_finalize();

    pub fn rb_check_type(_: VALUE, _: libc::c_int);
    pub fn rb_str_to_str(_: VALUE) -> VALUE;
    pub fn rb_string_value(_: *mut VALUE) -> VALUE;
    pub fn rb_string_value_ptr(_: *mut VALUE) -> *mut libc::c_char;
    pub fn rb_check_safe_obj(_: VALUE);
    pub fn rb_str_export(_: VALUE) -> VALUE;
    pub fn rb_str_export_locale(_: VALUE) -> VALUE;
    pub fn rb_get_path(_: VALUE) -> VALUE;
    pub fn rb_get_path_no_checksafe(_: VALUE) -> VALUE;
    pub fn rb_secure(_: libc::c_int);
    pub fn rb_safe_level() -> libc::c_int;
    pub fn rb_set_safe_level(_: libc::c_int);
    pub fn rb_set_safe_level_force(_: libc::c_int);

    pub fn rb_num2long(_: VALUE) -> libc::c_long;
    pub fn rb_num2ulong(_: VALUE) -> libc::c_ulong;

    pub fn rb_num2uint(_: VALUE) -> libc::c_ulong;
    pub fn rb_fix2uint(_: VALUE) -> libc::c_ulong;

    pub fn rb_num2short(_: VALUE) -> libc::c_short;
    pub fn rb_num2ushort(_: VALUE) -> libc::c_ushort;
    pub fn rb_fix2short(_: VALUE) -> libc::c_short;
    pub fn rb_fix2ushort(_: VALUE) -> libc::c_ushort;

    pub fn rb_newobj() -> VALUE;
    pub fn rb_newobj_of(_: VALUE, _: VALUE) -> VALUE;
    pub fn rb_obj_setup(obj: VALUE, klass: VALUE, ty: VALUE) -> VALUE;

    pub fn rb_float_new(_: libc::c_double) -> VALUE;
    pub fn rb_float_new_in_heap(_: libc::c_double) -> VALUE;
}
