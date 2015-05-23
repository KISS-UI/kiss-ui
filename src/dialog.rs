//! KISS-UI top-level dialogs (windows)

use super::BaseWidget;

use ::iup_sys;

use std::ffi::CString;
use std::ptr;

/// A top-level dialog that can create a new native window when shown,
/// and can contain a single widget (which can be a container for many widgets).
pub struct Dialog(BaseWidget);

impl Dialog {
    /// Create a new dialog with a single child. 
    ///
    /// To create a dialog containing multiple widgets, use a struct from the `container` module.
    ///
    /// ##Note
    /// This does **not** make the dialog appear on screen. `.show()` must be called after the
    /// dialog has been configured.
    ///
    /// ##Panics
    /// If called outside a valid KISS-UI context.
    pub fn new<W>(contents: W) -> Dialog where W: Into<BaseWidget> {
        assert_kiss_running!();

        unsafe { 
            let ptr = iup_sys::IupDialog(contents.into().0);
            Dialog(BaseWidget::from_ptr(ptr))
        }
    }

    /// Create a new dialog with no children.
    ///
    /// ##Panics
    /// If called outside a valid KISS-UI context.
    pub fn empty() -> Dialog {
        assert_kiss_running!();

        unsafe {
            let ptr = iup_sys::IupDialog(ptr::null_mut());
            Dialog(BaseWidget::from_ptr(ptr))
        }
    }

    /// Set the title of this dialog, which will appear in the title bar of the native window.
    pub fn set_title(mut self, title: &str) -> Self {
        self.set_str_attribute(::attrs::TITLE, title);
        self
    }

    /// Set the size of this dialog in pixels.
    pub fn set_size_pixels(mut self, width: u32, height: u32) -> Self {
        let rastersize = format!("{}x{}", width, height);
        self.set_str_attribute(::attrs::RASTERSIZE, rastersize);
        self
    }

    /// Get a child of this dialog named by `name`.
    ///
    /// Returns `None` if the child was not found.
    pub fn get_child(&self, name: &str) -> Option<BaseWidget> {
        let name = CString::new(name).unwrap();        

        unsafe {
            let child_ptr = iup_sys::IupGetDialogChild(self.ptr(), name.as_ptr());
            BaseWidget::from_ptr_opt(child_ptr)
        }
    }

    /// Destroy this dialog, deallocating it and all attached children.
    pub fn destroy(self) {
        self.0.destroy()
    }
}

impl_base_widget! { Dialog, Dialog, "dialog" }

impl_on_show! { Dialog }

pub fn popup_message_dialog<T: Into<Vec<u8>>, M: Into<Vec<u8>>>(title: T, message: M) {
    assert_kiss_running!();

    let title = CString::new(title).unwrap();
    let message = CString::new(message).unwrap();

    unsafe {
        iup_sys::IupMessage(title.as_ptr(), message.as_ptr());
    }
}
