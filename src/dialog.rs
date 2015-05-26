//! KISS-UI top-level dialogs (windows)

use widget_prelude::*;

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
            let ptr = iup_sys::IupDialog(contents.into().ptr());
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

/// Popup a message dialog and block until it is closed, by either the OK button or the exit
/// button.
pub fn message_popup<T: Into<String>, M: Into<String>>(title: T, message: M) {
    assert_kiss_running!();

    let title = CString::new(title.into()).unwrap();
    let message = CString::new(message.into()).unwrap();

    unsafe {
        iup_sys::IupMessage(title.as_ptr(), message.as_ptr());
    }
}

/// A builder for an alert dialog that can show a message and up to 3 buttons for the user's
/// response.
pub struct AlertPopupBuilder {
    pub title: String,
    pub message: String,
    pub button1: String,
    pub button2: Option<String>,
    pub button3: Option<String>,
}

impl AlertPopupBuilder {
    pub fn new<T: Into<String>, M: Into<String>, B1: Into<String>>(
        title: T, message: M, button1: B1
    ) -> AlertPopupBuilder {
        AlertPopupBuilder {
            title: title.into(),
            message: message.into(),
            button1: button1.into(),
            button2: None,
            button3: None,
        }
    }

    /// Set the text of the second button
    pub fn button2<B2: Into<String>>(mut self, button2: B2) -> Self {
        self.button2 = Some(button2.into());
        self
    }

    pub fn button3<B3: Into<String>>(mut self, button3: B3) -> Self {
        self.button3 = Some(button3.into());
        self
    }

    /// Popup the dialog and block until the user takes an action.
    ///
    /// Returns: which button was pressed, or **0** if the dialog was closed.
    pub fn popup(self) -> i32 {
        let title = CString::new(self.title).unwrap();
        let message = CString::new(self.message).unwrap();
        let button1 = CString::new(self.button1).unwrap();
        let button2 = self.button2.map(|b2| CString::new(b2).unwrap());
        let button3 = self.button3.map(|b3| CString::new(b3).unwrap());

        unsafe {
            iup_sys::IupAlarm(
                title.as_ptr(),
                message.as_ptr(),
                button1.as_ptr(),
                button2.as_ref().map_or_else(ptr::null, |b2| b2.as_ptr()),
                button3.as_ref().map_or_else(ptr::null, |b3| b3.as_ptr()),
            )
        }
    }
}
