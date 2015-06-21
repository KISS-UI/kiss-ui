//! A general widget type that can be specialized at runtime.

use widget_prelude::*;

use ::KISSContext;

use std::borrow::Borrow;

/// A general widget type that can be specialized at runtime via `Downcast`.
pub struct BaseWidget(IUPPtr);

impl BaseWidget {
    /// Attempt to load a widget named by `name` from internal storage.
    ///
    /// If successful, the `BaseWidget` can then be downcast to the original widget type.
    ///
    /// Returns `None` if no widget by that name was found.
    ///
    /// ##Panics
    /// If called before `kiss_ui::show_gui()` is invoked or after it returns.
    pub fn load<N: Borrow<str>>(name: N) -> Option<BaseWidget> {
        KISSContext::load_widget(&name) 
    }

    /// Attempt to downcast this `BaseWidget` to a more specialized widget type.
    ///
    /// This will return an error if the underlying widget class is different than the one 
    /// it is being cast to.
    pub fn try_downcast<T>(self) -> Result<T, Self> where T: Downcast {
        T::try_downcast(self) 
    }
}

impl_widget! { BaseWidget }

/// A trait describing a widget's ability to be downcast from `BaseWidget`.
pub trait Downcast: Widget {
    /// Attempt to downcast `base` to the `Self` type, 
    /// returning `Err(base)` if unsuccessful.
    fn try_downcast(base: BaseWidget) -> Result<Self, BaseWidget> {
        if Self::can_downcast(&base) {
            Ok(unsafe { Self::downcast(base) })
        } else {
            Err(base)
        }
    }

    // These are not meant for end-users to call.
    // They are an implementation detail of `try_downcast()`.
    #[doc(hidden)]
    unsafe fn downcast(base: BaseWidget) -> Self {
        Self::from_ptr(base.ptr())
    }

    #[doc(hidden)]
    fn can_downcast(base: &BaseWidget) -> bool;
}

