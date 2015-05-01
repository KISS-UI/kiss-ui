#![feature(collections, libc)]

extern crate libc;
extern crate iup_sys;

macro_rules! impl_base_widget {
    ($ty:ty) => (
        impl Into<::BaseWidget> for $ty {
            fn into(self) -> BaseWidget {
                self.0
            }
        }

        impl ::std::ops::Deref for $ty {
            type Target = BaseWidget;

            fn deref(&self) -> &BaseWidget {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut BaseWidget {
                &mut self.0
            }
        }
    )
}

// Internal use modules
mod attrs;
mod cstr_utils;

// User-facing modules
pub mod container;
pub mod dialog;
pub mod image;
pub mod text;

use cstr_utils::AsCStr;

use std::ffi::CString;
use std::ptr;

pub fn show_gui<F>(init_fn: F) where F: FnOnce() -> dialog::Dialog {
    unsafe { assert!(iup_sys::IupOpen(ptr::null(), ptr::null()) == 0); }
    init_fn().show();
    unsafe { 
        iup_sys::IupMainLoop();
        iup_sys::IupClose();
    }
}

pub trait Widget {
    fn show(&mut self);
    fn hide(&mut self);
}

pub struct BaseWidget(*mut iup_sys::Ihandle);

impl BaseWidget {
    pub unsafe fn null() -> BaseWidget {
        BaseWidget(ptr::null_mut())
    }

    fn from_ptr(ptr: *mut iup_sys::Ihandle) -> BaseWidget {
        assert!(!ptr.is_null());
        BaseWidget(ptr)
    }

    fn from_ptr_opt(ptr: *mut iup_sys::Ihandle) -> Option<BaseWidget> {
        if !ptr.is_null() {
            Some(BaseWidget(ptr))
        } else {
            None
        }
    }

    fn as_ptr(&self) -> *mut iup_sys::Ihandle {
        self.0
    }

    fn ptr_not_null(&self) -> *mut iup_sys::Ihandle {
        assert!(!self.0.is_null());
        self.0
    }

    fn set_str_attribute<V>(&mut self, name: &'static str, val: V) where V: Into<Vec<u8>> {
        let c_val = CString::new(val).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.ptr_not_null(), name.as_cstr(), c_val.as_ptr()); }
    }

    fn set_const_str_attribute(&mut self, name: &'static str, val: &'static str) {
        unsafe { iup_sys::IupSetAttribute(self.ptr_not_null(), name.as_cstr(), val.as_cstr()); }
    }

    fn set_attr_handle<H: Into<BaseWidget>>(&self, name: &'static str, handle: H) {
        unsafe { iup_sys::IupSetAttributeHandle(self.ptr_not_null(), name.as_cstr(), handle.into().ptr_not_null()); }
    }

    fn get_attr_handle(&self, name: &'static str) -> Option<BaseWidget> {
        let existing = unsafe { iup_sys::IupGetAttributeHandle(self.ptr_not_null(), name.as_cstr()) };
        BaseWidget::from_ptr_opt(existing)
    }

    fn destroy(self) {
        unsafe { iup_sys::IupDestroy(self.ptr_not_null()); }
    }

    pub fn show(&mut self) {
        unsafe { iup_sys::IupShow(self.ptr_not_null()); }
    }

    fn hide(&mut self) {
        unsafe { iup_sys::IupHide(self.ptr_not_null()); }
    }
}



