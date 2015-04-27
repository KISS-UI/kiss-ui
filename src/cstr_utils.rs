pub type CStrPtr = *const ::libc::c_char;

// Why does AsCStr: ?Sized not work?1
pub trait AsCStr {
    fn as_cstr(self) -> CStrPtr;
}

impl AsCStr for &'static str {
    fn as_cstr(self) -> CStrPtr {
        let bytes = self.as_bytes();
        assert!(bytes[bytes.len() - 1] == 0u8);
        bytes.as_ptr() as CStrPtr
    }
}
