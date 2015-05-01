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
        self.get_attr_handle(::attrs::IMAGE)
            .map(|img| img.destroy()); 

        self.set_attr_handle(::attrs::IMAGE, image);

        self
    }
}

impl_base_widget! { Label }
