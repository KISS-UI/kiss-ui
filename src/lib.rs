#![feature(associated_consts, collections, libc, scoped_tls)]

extern crate libc;
extern crate iup_sys;

macro_rules! impl_base_widget {
    ($ty:ty, $ty_cons:path, $classname:expr) => (
        impl Into<::BaseWidget> for $ty {
            fn into(self) -> ::BaseWidget {
                self.0
            }
        }

        impl ::std::ops::Deref for $ty {
            type Target = ::BaseWidget;

            fn deref(&self) -> &::BaseWidget {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut ::BaseWidget {
                &mut self.0
            }
        }

        impl ::Downcast for $ty {
            unsafe fn downcast(base: ::BaseWidget) -> $ty {
                $ty_cons(base)
            }

            fn classname() -> &'static str {
                $classname
            }
        }
    )
}

// Internal use modules
mod attrs;
mod cstr_utils;

// User-facing modules
#[macro_use]
pub mod callback;

pub mod button;
pub mod container;
pub mod dialog;
pub mod image;
pub mod text;

use cstr_utils::AsCStr;

use std::ffi::{CStr, CString};
use std::ptr;

pub fn show_gui<F>(init_fn: F) where F: FnOnce() -> dialog::Dialog {
    unsafe { 
        assert!(iup_sys::IupOpen(ptr::null(), ptr::null()) == 0);
        // Force IUP to always use UTF-8
        iup_sys::IupSetGlobal(::attrs::UTF8_MODE.as_cstr(), ::attrs::values::YES.as_cstr());
    }

    init_fn().show();

    unsafe { 
        iup_sys::IupMainLoop();
        iup_sys::IupClose();
    }
}

// This struct is defined here so its private methods are available to submodules

pub struct BaseWidget(*mut iup_sys::Ihandle);

impl BaseWidget {
    pub unsafe fn null() -> BaseWidget {
        BaseWidget(ptr::null_mut())
    }

    pub unsafe fn from_ptr(ptr: *mut iup_sys::Ihandle) -> BaseWidget {
        assert!(!ptr.is_null());
        BaseWidget(ptr)
    }

    pub unsafe fn from_ptr_opt(ptr: *mut iup_sys::Ihandle) -> Option<BaseWidget> {
        if !ptr.is_null() {
            Some(BaseWidget(ptr))
        } else {
            None
        }
    }

    fn ptr(&self) -> *mut iup_sys::Ihandle {
        self.0
    } 

    fn set_str_attribute<V>(&mut self, name: &'static str, val: V) where V: Into<Vec<u8>> {
        let c_val = CString::new(val).unwrap();
        unsafe { iup_sys::IupSetStrAttribute(self.ptr(), name.as_cstr(), c_val.as_ptr()); }
    }

    fn set_opt_str_attribute<V>(&mut self, name: &'static str, val: Option<V>) where V: Into<Vec<u8>> {
        let c_val = val.map(CString::new).map(Result::unwrap);
        unsafe { 
            iup_sys::IupSetStrAttribute(
                self.ptr(),
                name.as_cstr(),
                // This looks backwards, but check the docs. It's right.
                c_val.as_ref().map_or_else(ptr::null, |c_val| c_val.as_ptr())
            )
        }
    }

    fn set_const_str_attribute(&mut self, name: &'static str, val: &'static str) {
        unsafe { iup_sys::IupSetAttribute(self.ptr(), name.as_cstr(), val.as_cstr()); }
    }

    fn get_str_attribute(&self, name: &'static str) -> Option<&str> {
        let ptr = unsafe { iup_sys::IupGetAttribute(self.ptr(), name.as_cstr()) };

        if !ptr.is_null() {
            unsafe {
                // Safe since we're controlling the lifetime
                let c_str = CStr::from_ptr(ptr);
                // We're forcing IUP to use UTF-8 
                Some(::std::str::from_utf8_unchecked(c_str.to_bytes()))
            }
        } else {
            None
        }
    }

    fn set_int_attribute(&mut self, name: &'static str, val: i32) {
        unsafe { iup_sys::IupSetInt(self.ptr(), name.as_cstr(), val); }
    }

    fn get_int_attribute(&self, name: &'static str) -> i32 {
        unsafe { iup_sys::IupGetInt(self.ptr(), name.as_cstr()) }
    }

    fn get_int2_attribute(&self, name: &'static str) -> (i32, i32) {
        let mut left = 0;
        let mut right = 0;

        unsafe { 
            assert!(iup_sys::IupGetIntInt(self.ptr(), name.as_cstr(), &mut left, &mut right) != 0); 
        }

        (left, right)
    }

    fn set_bool_attribute(&mut self, name: &'static str, val: bool) {
        let val = ::attrs::values::bool_yes_no(val);
        self.set_const_str_attribute(name, val);        
    }

    fn set_attr_handle<H: Into<BaseWidget>>(&self, name: &'static str, handle: H) {
        unsafe { iup_sys::IupSetAttributeHandle(self.ptr(), name.as_cstr(), handle.into().ptr()); }
    }

    fn get_attr_handle(&self, name: &'static str) -> Option<BaseWidget> {
        unsafe { 
            let existing = iup_sys::IupGetAttributeHandle(self.ptr(), name.as_cstr());
            BaseWidget::from_ptr_opt(existing)
        }
    }

    fn set_callback(&mut self, name: &'static str, callback: ::iup_sys::Icallback) {
        unsafe { iup_sys::IupSetCallback(self.ptr(), name.as_cstr(), callback); } 
    } 

    fn destroy(self) {
        unsafe { iup_sys::IupDestroy(self.ptr()); }
    }

    pub fn show(&mut self) {
        unsafe { iup_sys::IupShow(self.ptr()); }
    }

    pub fn hide(&mut self) {
        unsafe { iup_sys::IupHide(self.ptr()); }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.set_bool_attribute(::attrs::ACTIVE, enabled);
    }

    pub fn set_name(&mut self, name: &str) {
        self.set_str_attribute(::attrs::NAME, name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.get_str_attribute(::attrs::NAME) 
    }

    pub fn try_downcast<T>(self) -> Result<T, Self> where T: Downcast {
        T::try_downcast(self) 
    }

    pub fn get_sibling(&self) -> Option<BaseWidget> {
        unsafe {
            let ptr = iup_sys::IupGetBrother(self.ptr());
            Self::from_ptr_opt(ptr)
        }
    }

    pub fn get_parent(&self) -> Option<BaseWidget> {
        unsafe {
            let ptr = iup_sys::IupGetParent(self.ptr());
            Self::from_ptr_opt(ptr)
        }
    }

    pub fn get_dialog(&self) -> Option<dialog::Dialog> {
        unsafe {
            let ptr = iup_sys::IupGetDialog(self.ptr());
            Self::from_ptr_opt(ptr).map(|base| dialog::Dialog::downcast(base))
        }
    }

    fn classname(&self) -> &CStr {
        unsafe { CStr::from_ptr(iup_sys::IupGetClassName(self.ptr())) } 
    }
}

pub trait Downcast: Into<BaseWidget> {
    fn try_downcast(base: BaseWidget) -> Result<Self, BaseWidget> {
        if Self::classname().as_bytes() == base.classname().to_bytes() {
            Ok(unsafe { Self::downcast(base) })
        } else {
            Err(base)
        }
    }

    unsafe fn downcast(base: BaseWidget) -> Self;
    fn classname() -> &'static str;
}
