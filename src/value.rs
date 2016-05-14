use ffi;

pub struct Value(ffi::VALUE);

impl From<ffi::VALUE> for Value
{
    fn from(value: ffi::VALUE) -> Value {
        Value(value)
    }
}
