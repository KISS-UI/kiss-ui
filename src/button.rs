//! Buttons that can receive user input.

use widget_prelude::*;

use std::ptr;

/// A button that can be clicked momentarily and invoke a callback when this happens.
pub struct Button(IUPPtr);

impl Button {
    /// Create a new `Button` with no label.
    pub fn new() -> Button {
        unsafe {
            let ptr = ::iup_sys::IupButton(ptr::null(), ptr::null());
            Self::from_ptr(ptr)
        }
    }

    /// Set the label of this button. Can be blank.
    pub fn set_label<L: Into<String>>(self, label: L) -> Self {
        self.set_str_attribute(::attrs::TITLE, label);
        self        
    }
}

impl_widget! { Button, "button" }

impl_onclick! { Button }

impl ::image::ImageContainer for Button {}
