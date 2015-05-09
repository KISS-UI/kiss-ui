use super::BaseWidget;

use ::iup_sys;

use std::ffi::CString;

pub struct Dialog(BaseWidget);

impl Dialog {
    pub fn new<W>(contents: W) -> Dialog where W: Into<BaseWidget> {
        unsafe { 
            let ptr = iup_sys::IupDialog(contents.into().0);
            Dialog(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.set_str_attribute(::attrs::TITLE, title);
        self
    }

    pub fn set_size_pixels(mut self, width: u32, height: u32) -> Self {
        let rastersize = format!("{}x{}", width, height);
        self.set_str_attribute(::attrs::RASTERSIZE, rastersize);
        self
    }

    pub fn get_child(&self, name: &str) -> Option<BaseWidget> {
        let name = CString::new(name).unwrap();        

        unsafe {
            let child_ptr = iup_sys::IupGetDialogChild(self.ptr(), name.as_ptr());
            BaseWidget::from_ptr_opt(child_ptr)
        }
    }


}

impl_base_widget! { Dialog, Dialog, "dialog" }

impl_on_show! { Dialog }

pub fn popup_message_dialog<T: Into<Vec<u8>>, M: Into<Vec<u8>>>(title: T, message: M) {
    let title = CString::new(title).unwrap();
    let message = CString::new(message).unwrap();

    unsafe {
        iup_sys::IupMessage(title.as_ptr(), message.as_ptr());
    }
}
