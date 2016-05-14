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
