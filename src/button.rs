//! Buttons that can receive user input.

use widget_prelude::*;

use std::ptr;

/// A button that can be clicked momentarily and invoke a callback when this happens.
pub struct Button(BaseWidget);

impl Button {
    /// Create a new `Button` with no label.
    pub fn new() -> Button {
        unsafe {
            let ptr = ::iup_sys::IupButton(ptr::null(), ptr::null());
            Button(BaseWidget::from_ptr(ptr))
        }
    }

    /// Set the label of this button if `Some`, remove it otherwise.
    pub fn set_label<L: Into<String>>(mut self, label: Option<L>) -> Self {
        self.set_opt_str_attribute(::attrs::TITLE, label);
        self        
    }
}

impl_base_widget! { Button, Button, "button" }

impl_onclick! { Button }

impl ::image::ImageContainer for Button {}
