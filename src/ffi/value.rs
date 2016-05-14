use libc;
use std;

#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct VALUE(pub libc::uintptr_t);

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
