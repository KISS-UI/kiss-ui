use super::BaseWidget;

use std::ffi::CString;
use std::ptr;

pub struct Label(BaseWidget);

impl Label {
    pub fn new<S: Into<Vec<u8>>>(text: S) -> Label {
        let c_text = CString::new(text).unwrap();
         unsafe {
            let ptr = ::iup_sys::IupLabel(c_text.as_ptr());
            Label(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn new_empty() -> Label {
        unsafe { 
            let ptr = ::iup_sys::IupLabel(ptr::null());
            Label(BaseWidget::from_ptr(ptr))       
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.set_str_attribute(::attrs::TITLE, text);
    }

    pub fn get_text(&self) -> &str {
        self.get_str_attribute(::attrs::TITLE).unwrap_or("")
    }
}

impl_base_widget! { Label, Label, "label" }

impl ::image::ImageContainer for Label {}

pub struct TextBox(BaseWidget);

impl TextBox {
    pub fn new() -> TextBox {
        unsafe {
            let ptr = ::iup_sys::IupText(ptr::null());
            TextBox(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_multiline(mut self, multiline: bool) -> Self {
        self.set_bool_attribute(::attrs::MULTILINE, multiline);
        self
    }

    pub fn set_visible_columns(mut self, cols: u32) -> Self {
        self.set_int_attribute(::attrs::VISIBLE_COLUMNS, cols as i32);
        self
    }

    pub fn set_visible_lines(mut self, lines: u32) -> Self {
        self.set_int_attribute(::attrs::VISIBLE_LINES, lines as i32);
        self
    }

    pub fn set_text(&mut self, value: &str) {
        self.set_str_attribute(::attrs::VALUE, value);
    }

    pub fn get_text(&self) -> &str {
        self.get_str_attribute(::attrs::VALUE).unwrap_or("")
    }    
}

impl_base_widget! { TextBox, TextBox, "text" }

impl_on_value_change! { TextBox }

