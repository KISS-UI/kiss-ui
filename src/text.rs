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
        Label(baseWidget::from_ptr(ptr))
    }

    pub fn set_image(self, image: ::image::Image) -> Label {
        let ptr = image.0.as_ptr();
        unsafe { iup_sys::IupSetAttributeHandle(self.0.as_ptr(), ::attrs::IMAGE, 
    }
}

impl_into_base_widget! { Label }
