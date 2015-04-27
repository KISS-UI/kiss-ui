use super::BaseWidget;

use std::ffi::CString;

pub struct Label(BaseWidget);

impl Label {
    pub fn new<S: Into<Vec<u8>>>(text: S) -> Label {
        let c_text = CString::new(text).unwrap();
        let ptr = unsafe { ::iup_sys::IupLabel(c_text.as_ptr()) };
        Label(BaseWidget::from_ptr(ptr))
    }
}

impl_into_base_widget! { Label }
