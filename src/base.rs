use utils::cstr::AsCStr;

use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::ptr;

/// Defines operations common to all widget types.
///
/// Some operations may have no effect for certain widget types.
///
/// All widgets must implement `Deref` and `DerefMut` with this type as the target.
pub struct BaseWidget(*mut iup_sys::Ihandle);

impl BaseWidget { 
    unsafe fn from_ptr(ptr: *mut iup_sys::Ihandle) -> BaseWidget {
        assert!(!ptr.is_null());
        BaseWidget(ptr)
    }

    unsafe fn from_ptr_opt(ptr: *mut iup_sys::Ihandle) -> Option<BaseWidget> {
        if !ptr.is_null() {
            Some(BaseWidget(ptr))
        } else {
            None
        }
    }

    fn ptr(&self) -> *mut iup_sys::Ihandle {
        self.0
    } 

    fn set_str_attribute<V>(&mut self, name: &'static str, val: V) where V: Into<String> {
        let c_val = CString::new(val.into()).unwrap();
        unsafe { iup_sys::IupSetStrAttribute(self.ptr(), name.as_cstr(), c_val.as_ptr()); }
    }

    fn set_opt_str_attribute<V>(&mut self, name: &'static str, val: Option<V>) where V: Into<String> {
        let c_val = val.map(V::into).map(CString::new).map(Result::unwrap);
        unsafe { 
            iup_sys::IupSetStrAttribute(
                self.ptr(),
                name.as_cstr(),
                // This looks backwards, but check the docs. It's right.
                c_val.as_ref().map_or_else(ptr::null, |c_val| c_val.as_ptr())
            )
        }
    }

    fn set_const_str_attribute(&mut self, name: &'static str, val: &'static str) {
        unsafe { iup_sys::IupSetAttribute(self.ptr(), name.as_cstr(), val.as_cstr()); }
    }

    fn get_str_attribute(&self, name: &'static str) -> Option<&str> {
        let ptr = unsafe { iup_sys::IupGetAttribute(self.ptr(), name.as_cstr()) };

        if !ptr.is_null() {
            unsafe {
                // Safe since we're controlling the lifetime
                let c_str = CStr::from_ptr(ptr);
                // We're forcing IUP to use UTF-8 
                Some(::std::str::from_utf8_unchecked(c_str.to_bytes()))
            }
        } else {
            None
        }
    }

    fn set_int_attribute(&mut self, name: &'static str, val: i32) {
        unsafe { iup_sys::IupSetInt(self.ptr(), name.as_cstr(), val); }
    }

    fn get_int_attribute(&self, name: &'static str) -> i32 {
        unsafe { iup_sys::IupGetInt(self.ptr(), name.as_cstr()) }
    }

    fn get_int2_attribute(&self, name: &'static str) -> (i32, i32) {
        let mut left = 0;
        let mut right = 0;

        unsafe { 
            assert!(iup_sys::IupGetIntInt(self.ptr(), name.as_cstr(), &mut left, &mut right) != 0); 
        }

        (left, right)
    }

    fn set_float_attribute(&mut self, name: &'static str, val: f32) {
        unsafe { iup_sys::IupSetFloat(self.ptr(), name.as_cstr(), val); } 
    }

    fn get_float_attribute(&self, name: &'static str) -> f32 {
        unsafe { iup_sys::IupGetFloat(self.ptr(), name.as_cstr()) }
    }

    fn set_bool_attribute(&mut self, name: &'static str, val: bool) {
        let val = ::attrs::values::bool_yes_no(val);
        self.set_const_str_attribute(name, val);        
    }

    fn set_attr_handle<H: Into<BaseWidget>>(&self, name: &'static str, handle: H) {
        unsafe { iup_sys::IupSetAttributeHandle(self.ptr(), name.as_cstr(), handle.into().ptr()); }
    }

    fn get_attr_handle(&self, name: &'static str) -> Option<BaseWidget> {
        unsafe { 
            let existing = iup_sys::IupGetAttributeHandle(self.ptr(), name.as_cstr());
            BaseWidget::from_ptr_opt(existing)
        }
    }

    // Should this be `unsafe` since `callback` is a pointer with no lifetime guarantees?
    fn set_callback(&mut self, name: &'static str, callback: ::iup_sys::Icallback) {
        unsafe { iup_sys::IupSetCallback(self.ptr(), name.as_cstr(), callback); } 
    } 

    // Should only be exposed by widget types that need it.
    fn destroy(self) {
        unsafe { iup_sys::IupDestroy(self.ptr()); }
    }

    /// Show this widget if it was previously hidden.
    ///
    /// Does nothing if the widget is already shown, or if the operation does not apply.
    pub fn show(&mut self) {
        unsafe { iup_sys::IupShow(self.ptr()); }
    }

    /// Hide this widget if it was previously visible.
    ///
    /// Does nothing if the widget is already hidden, or if the operation does not apply.
    pub fn hide(&mut self) {
        unsafe { iup_sys::IupHide(self.ptr()); }
    }

    /// Set the widget's visibility state.
    ///
    /// `.set_visible(true)` is equivalent to calling `.show()`, and `.set_visible(false)`
    /// is equivalent to calling `.hide()`.
    ///
    /// Does nothing if the widget is in the same visibility state as the one being set,
    /// or if the operation does not apply.
    pub fn set_visible(&mut self, visible: bool) {
        self.set_bool_attribute(::attrs::VISIBLE, visible);
    }

    /// Set the widget's enabled state.
    ///
    /// When a widget is disabled, it does not react to user interaction or invoke any callbacks.
    ///
    /// Does nothing if the widget does not support being disabled.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.set_bool_attribute(::attrs::ACTIVE, enabled);
    }

    /// Set the position of this widget relative to the top-left corner of its parent.
    ///
    /// Does nothing if the widget is not renderable or not attached to a parent.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.set_str_attribute(::attrs::POSITION, format!("{x},{y}", x=x, y=y));
    }

    /// Get the position of this widget relative to the top-left corner of its parent.
    ///
    /// Returns (0, 0) if the widget is not renderable, not attached to a parent, or if that is the
    /// widget's actual relative position.
    pub fn get_position(&self) -> (i32, i32) {
        self.get_int2_attribute(::attrs::POSITION)
    }

    /// Set the name of the widget so it can be found within its parent.
    ///
    /// Does nothing if the widget does not support having a name.
    pub fn set_name(&mut self, name: &str) {
        self.set_str_attribute(::attrs::NAME, name);
    }

    /// Get the name of this widget, if the widget supports having a name and one is set.
    pub fn get_name(&self) -> Option<&str> {
        self.get_str_attribute(::attrs::NAME) 
    }  

    /// Get the next child in the parent after this widget, based on the order in which they were 
    /// added.
    ///
    /// Returns `None` if this widget is an only child or is not attached to a parent.
    pub fn get_sibling(&self) -> Option<BaseWidget> {
        unsafe {
            let ptr = iup_sys::IupGetBrother(self.ptr());
            Self::from_ptr_opt(ptr)
        }
    }

    /// Get the parent of this widget.
    ///
    /// Returns `None` if this widget has no parent.
    pub fn get_parent(&self) -> Option<BaseWidget> {
        unsafe {
            let ptr = iup_sys::IupGetParent(self.ptr());
            Self::from_ptr_opt(ptr)
        }
    }

    /// Get the containing dialog of this widget.
    ///
    /// Returns `None` if this widget is not attached to a dialog.
    pub fn get_dialog(&self) -> Option<dialog::Dialog> {
        unsafe {
            let ptr = iup_sys::IupGetDialog(self.ptr());
            Self::from_ptr_opt(ptr).map(|base| dialog::Dialog::downcast(base))
        }
    }

    /// Get the rendered size of this widget, in pixels.
    ///
    /// Returns `(0, 0)` if this widget has no rendered size.
    pub fn get_size_pixels(&self) -> (u32, u32) {
        let (width, height) = self.get_int2_attribute(::attrs::RASTERSIZE);
        (width as u32, height as u32)
    }

    /// Store this widget under `name`, returning the previous widget stored, if any.
    ///
    /// It may later be retrieved from any valid KISS-UI context 
    /// by calling `BaseWidget::load(name)`.
    pub fn store<N: Into<String>>(&self, name: N) -> Option<BaseWidget> {
        WIDGET_STORE.with(|store| {
            store.borrow_mut().insert(name.into(), self.clone())
        })
    }

    /// Attempt to load a widget named by `name` from internal storage.
    ///
    /// If successful, the `BaseWidget` can then be downcast to the original widget type.
    ///
    /// Returns `None` if no widget by that name was found.
    ///
    /// ##Panics
    /// If called before `kiss_ui::show_gui()` is invoked or after it returns.
    pub fn load<N: Borrow<str>>(name: N) -> Option<BaseWidget> {
        assert_kiss_running!();

        WIDGET_STORE.with(|store| {
            store.borrow().get(name.borrow()).cloned()
        })
    }

    /// Attempt to downcast this `BaseWidget` to a more specialized widget type.
    ///
    /// This will return an error if the underlying widget class is different than the one 
    /// it is being cast to.
    pub fn try_downcast<T>(self) -> Result<T, Self> where T: Downcast {
        T::try_downcast(self) 
    }

    fn classname(&self) -> &CStr {
        unsafe { CStr::from_ptr(iup_sys::IupGetClassName(self.ptr())) } 
    }
}

impl Clone for BaseWidget {
    fn clone(&self) -> Self {
        BaseWidget(self.0)
    }
}

/// A trait describing a widget's ability to be downcast from `BaseWidget`.
pub trait Downcast {
    /// Attempt to downcast `base` to the `Self` type, 
    /// returning `Err(base)` if unsuccessful.
    fn try_downcast(base: BaseWidget) -> Result<Self, BaseWidget>;
}

/// Implementation detail; please ignore.
impl<T> Downcast for T where T: _Downcast {
    fn try_downcast(base: BaseWidget) -> Result<Self, BaseWidget> {
        if Self::classname().as_bytes() == base.classname().to_bytes() {
            Ok(unsafe { Self::downcast(base) })
        } else {
            Err(base)
        }
    }
}

/// Private trait to be implemented by all widget types.
trait _Downcast: Into<BaseWidget> {
    unsafe fn downcast(base: BaseWidget) -> Self;
    fn classname() -> &'static str;
}

// Placed here so submodules can access the private `as_cstr` method.
/// Semantics differ based on the method that takes this enum.
#[derive(Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {
    fn as_cstr(self) -> &'static str {
        use self::Orientation::*;

        match self {
            Vertical => cstr!("VERTICAL"),
            Horizontal => cstr!("HORIZONTAL"),
        }
    }
}
