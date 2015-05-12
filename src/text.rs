//! Widgets that can render and process text (labels, text boxes).

use super::BaseWidget;

use std::ffi::CString;
use std::ptr;

/// A static widget that renders text within its parent.
pub struct Label(BaseWidget);

impl Label {
    /// Create a label with some text. 
    pub fn new<S: Into<String>>(text: S) -> Label {
        let c_text = CString::new(text.into()).unwrap();
         unsafe {
            let ptr = ::iup_sys::IupLabel(c_text.as_ptr());
            Label(BaseWidget::from_ptr(ptr))
        }
    }

    /// Create a blank label. The text can be set later.
    pub fn new_empty() -> Label {
        unsafe { 
            let ptr = ::iup_sys::IupLabel(ptr::null());
            Label(BaseWidget::from_ptr(ptr))       
        }
    }

    /// Update the text of this label.
    pub fn set_text(&mut self, text: &str) {
        self.set_str_attribute(::attrs::TITLE, text);
    }

    /// Get the text of this label.
    pub fn get_text(&self) -> &str {
        self.get_str_attribute(::attrs::TITLE).unwrap_or("")
    }
}

impl_base_widget! { Label, Label, "label" }

impl ::image::ImageContainer for Label {}

/// A widget that renders user-editable text.
pub struct TextBox(BaseWidget);

impl TextBox {
    /// Create a new, empty text box.
    pub fn new() -> TextBox {
        unsafe {
            let ptr = ::iup_sys::IupText(ptr::null());
            TextBox(BaseWidget::from_ptr(ptr))
        }
    }

    /// Set if the text box should accept and render newline characters.
    ///
    /// If `false`, it will only be slightly taller than a line of text in the current font.
    /// If `true`, the total dimensions will be set by `set_visible_columns` and
    /// `set_visible_lines`. Text outside these bounds will be accessible with a scrollbar.
    pub fn set_multiline(mut self, multiline: bool) -> Self {
        self.set_bool_attribute(::attrs::MULTILINE, multiline);
        self
    }

    /// Set the rendered width of the textbox in columns (character width + padding).
    ///
    /// If the textbox is set as multiline, this will cause additional text beyond the maximum
    /// width to wrap. Otherwise, it can be scrolled only horizontally.
    pub fn set_visible_columns(mut self, cols: u32) -> Self {
        self.set_int_attribute(::attrs::VISIBLE_COLUMNS, cols as i32);
        self
    }

    /// Set the rendered height of the textbox in lines (character height + padding).
    ///
    /// If the textbox is set as multiline, newline characters will push text following them to the
    /// next visible line. Line counts beyond these bounds will cause a scrollbar to be shown.
    pub fn set_visible_lines(mut self, lines: u32) -> Self {
        self.set_int_attribute(::attrs::VISIBLE_LINES, lines as i32);
        self
    }

    /// Set the text of this textbox.
    pub fn set_text(&mut self, value: &str) {
        self.set_str_attribute(::attrs::VALUE, value);
    }

    /// Get the text value of this textbox.
    pub fn get_text(&self) -> &str {
        self.get_str_attribute(::attrs::VALUE).unwrap_or("")
    }    
}

impl_base_widget! { TextBox, TextBox, "text" }

impl_on_value_change! { TextBox }

