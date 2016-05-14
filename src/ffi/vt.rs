pub use self::value_type::*;

#[repr(i32)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum value_type {
    T_NONE     = 0x00,
    T_OBJECT   = 0x01,
    T_CLASS    = 0x02,
    T_MODULE   = 0x03,
    T_FLOAT    = 0x04,
    T_STRING   = 0x05,
    T_REGEXP   = 0x06,
    T_ARRAY    = 0x07,
    T_HASH     = 0x08,
    T_STRUCT   = 0x09,
    T_BIGNUM   = 0x0a,
    T_FILE     = 0x0b,
    T_DATA     = 0x0c,
    T_MATCH    = 0x0d,
    T_COMPLEX  = 0x0e,
    T_RATIONAL = 0x0f,
    T_NIL      = 0x11,
    T_TRUE     = 0x12,
    T_FALSE    = 0x13,
    T_SYMBOL   = 0x14,
    T_FIXNUM   = 0x15,
    T_UNDEF    = 0x1b,
    T_NODE     = 0x1c,
    T_ICLASS   = 0x1d,
    T_ZOMBIE   = 0x1e,
    T_MASK     = 0x1f
}
