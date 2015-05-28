//! Progress bars and dialogs.

use widget_prelude::*;

use container::Orientation;

/// A widget that renders a bar which fills as its set value approaches a maximum.
///
/// For more info, see the [`IupProgressBar`][iup-progress] documentation. (Note: "marquee" is the
/// same as "indefinite")
///
/// [iup-progress]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupprogressbar.html
pub struct ProgressBar(IUPPtr);

impl ProgressBar {
    /// Create a new progress bar.
    pub fn new() -> ProgressBar {
        unsafe {
            let ptr = ::iup_sys::IupProgressBar();
            Self::from_ptr(ptr)
        }
    }

    /// Set this progress bar as indefinite or not. 
    ///
    /// In the indefinite state, the progress bar will not
    /// show its true value; instead it will render a looping animation.
    ///
    /// This may not have a visual effect on certain platforms. 
    pub fn set_indefinite(self, is_indefinite: bool) -> Self {
        self.set_bool_attribute(::attrs::MARQUEE, is_indefinite);
        self
    }

    /// Set if the progress bar should render solid (`false`) or dashed (`true`).
    ///
    /// This may not have a visual effect on certain platforms.
    pub fn set_dashed(self, dashed: bool) -> Self {
        self.set_bool_attribute(::attrs::DASHED, dashed);
        self
    }

    /// Set the maximum value of this progress bar, i.e. the value at which it will show full.
    ///
    /// Defaults to `1.0`.
    pub fn set_max(self, max: f32) -> Self {
        self.set_float_attribute(::attrs::MAX, max);
        self
    }

    /// Set the minimum value of this progress bar, i.e. the value at which it will be empty.
    ///
    /// Defaults to `0.0`.
    pub fn set_min(self, min: f32) -> Self {
        self.set_float_attribute(::attrs::MIN, min);
        self
    }

    /// Set the orientation of this progress bar.
    ///
    /// * `Vertical`: The progress bar will render as a vertical bar, and fill from bottom to top.
    /// * `Horizontal`: The progress bar will render as a horizontal bar, and fill from left to
    /// right.
    pub fn set_orientation(self, orientation: Orientation) -> Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, orientation.as_cstr());
        self
    } 

    /// Set the current value of this progress bar. Its rendered infill will be updated to reflect
    /// the new value in relation to the minimum and maximum.
    pub fn set_value(self, val: f32) -> Self {
        self.set_float_attribute(::attrs::VALUE, val);
        self
    }
    
    /// Get the current value.
    pub fn get_value(self) -> f32 {
        self.get_float_attribute(::attrs::VALUE)
    }

    /// Add `amt` to the current value and update it. `amt` may be negative. 
    pub fn add_value(self, amt: f32) -> Self {
        let val = self.get_float_attribute(::attrs::VALUE);
        self.set_float_attribute(::attrs::VALUE, val + amt);
        self
    }
}

impl_widget! { ProgressBar, "progressbar" }

