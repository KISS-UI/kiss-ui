use super::BaseWidget;

use std::ffi::CString;

pub struct Label(BaseWidget);

impl Label {
    pub fn new<S: Into<Vec<u8>>>(text: S) -> Label {
        let c_text = CString::new(text).unwrap();
        let ptr = unsafe { ::iup_sys::IupLabel(c_text.as_ptr()) };
        Label(BaseWidget::from_ptr(ptr))
    }

    pub fn new_empty() -> Label {
        use std::ptr;
        let ptr = unsafe { ::iup_sys::IupLabel(ptr::null()) };
        Label(BaseWidget::from_ptr(ptr))
    }

    pub fn set_image(self, image: ::image::Image) -> Label {
        // Deallocate the existing image if there is one.
        let existing = unsafe { ::iup_sys::IupGetAttributeHandle(self.0.as_ptr(), ::attrs::IMAGE) };    
        if !existing.is_null() {
            BaseWidget::from_ptr(existing).destroy();
        }

        unsafe { ::iup_sys::IupSetAttributeHandle(self.0.as_ptr(), ::attrs::IMAGE, image.0.as_ptr()); }

        self
    }
}

impl_base_widget! { Label }
