#![feature(collections, libc)]

extern crate libc;
extern crate iup_sys;

macro_rules! impl_into_base_widget {
    ($ty:ty) => (
        impl Into<::BaseWidget> for $ty {
            fn into(self) -> BaseWidget {
                self.0
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

    fn as_ptr(&self) -> *mut iup_sys::Ihandle {
        self.0
    }

    fn set_str_attribute<V>(&mut self, name: &'static str, val: V) where V: Into<Vec<u8>> {
        assert!(!self.0.is_null());
        let c_val = CString::new(val).unwrap();
        unsafe { iup_sys::IupSetAttribute(self.as_ptr(), name.as_cstr(), c_val.as_ptr()); }
    }

    fn set_const_str_attribute(&mut self, name: &'static str, val: &'static str) {
        assert!(!self.0.is_null());
        unsafe { iup_sys::IupSetAttribute(self.as_ptr(), name.as_cstr(), val.as_cstr()); }
    }
}

impl Widget for BaseWidget {
    fn show(&mut self) {
        unsafe { iup_sys::IupShow(self.0); }  
    }

    fn hide(&mut self) {
        unsafe { iup_sys::IupHide(self.0); }
    }
}

impl<W: AsMut<BaseWidget>> Widget for W {
    fn show(&mut self) {
        self.as_mut().show()
    }

    fn hide(&mut self) {
        self.as_mut().hide()
    }
}



