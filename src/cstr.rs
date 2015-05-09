pub trait AsCStr {
    fn as_cstr(&self) -> *const ::libc::c_char;
}

impl AsCStr for &'static str {
    fn as_cstr(&self) -> *const ::libc::c_char {
        let bytes = self.as_bytes();
        assert!(bytes[bytes.len() - 1] == 0u8);
        bytes.as_ptr() as *const ::libc::c_char
    }
}

macro_rules! cstr (
    ($val:expr) => (
        concat!($val, "\0")
    )
);

macro_rules! c_str_const (
    ($name:ident = $val:expr) => (
        pub const $name: &'static str = cstr!($val);
    )
);

macro_rules! c_str_consts {
    ($($name:ident = $val:expr),+,) => (
        $(c_str_const!($name = $val);)+
    )
}
