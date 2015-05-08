use super::BaseWidget;

use std::ptr;

pub struct Button(BaseWidget);

impl Button {
    pub fn new() -> Button {
        unsafe {
            let ptr = ::iup_sys::IupButton(ptr::null(), ptr::null());
            Button(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_label<L: Into<Vec<u8>>>(mut self, label: Option<L>) -> Self {
        self.set_opt_str_attribute(::attrs::TITLE, label);
        self        
    }
}

impl_base_widget! { Button, Button, "button" }

impl_onclick! { Button }

impl ::image::ImageContainer for Button {}
