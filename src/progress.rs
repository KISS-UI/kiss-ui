//! Progress bars and dialogs.

use ::BaseWidget;

/// A widget that renders a bar which fills as its set value approaches a maximum.
pub struct ProgressBar(BaseWidget);

impl ProgressBar {
    /// Create a new progress bar.
    pub fn new() -> ProgressBar {
        unsafe {
            let ptr = ::iup_sys::IupProgressBar();
            ProgressBar(BaseWidget::from_ptr(ptr))
        }
    }

    /// Set this progress bar as indefinite or not. 
    ///
    /// In the indefinite state, the progress bar will not
    /// show its true value; instead it will render a looping animation.
    ///
    /// This may not have a visual effect on certain platforms. 
    pub fn set_indefinite(mut self, is_indefinite: bool) -> Self {
        self.set_bool_attribute(::attrs::MARQUEE, is_indefinite);
        self
    }

    /// Set if the progress bar should render solid (`false`) or dashed (`true`).
    ///
    /// This may not have a visual effect on certain platforms.
    pub fn set_dashed(mut self, dashed: bool) -> Self {
        self.set_bool_attribute(::attrs::DASHED, dashed);
        self
    }

    /// Set the maximum value of this progress bar, i.e. the value at which it will show full.
    ///
    /// Defaults to `1.0`.
    pub fn set_max(mut self, max: f32) -> Self {
        self.set_float_attribute(::attrs::MAX, max);
        self
    }

    /// Set the minimum value of this progress bar, i.e. the value at which it will be empty.
    ///
    /// Defaults to `0.0`.
    pub fn set_min(mut self, min: f32) -> Self {
        self.set_float_attribute(::attrs::MIN, min);
        self
    }

    /// Set this progress bar to render vertically. It will fill from bottom to top.
    pub fn set_vertical(mut self) -> Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, ::attrs::values::VERTICAL);
        self
    }

    /// Set this progress bar to render horizontally. It will fill from left to right. (Default)
    pub fn set_horizontal(mut self) -> Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, ::attrs::values::HORIZONTAL);
        self
    }

    /// Set the current value of this progress bar. Its rendered infill will be updated to reflect
    /// the new value in relation to the minimum and maximum.
    pub fn set_value(&mut self, val: f32) {
        self.set_float_attribute(::attrs::VALUE, val);        
    }
    
    /// Get the current value.
    pub fn get_value(&self) -> f32 {
        self.get_float_attribute(::attrs::VALUE)
    }

    /// Add `amt` to the current value and update it. `amt` may be negative. 
    pub fn add_value(&mut self, amt: f32) {
        let val = self.get_float_attribute(::attrs::VALUE);
        self.set_float_attribute(::attrs::VALUE, val + amt);
    }
}

impl_base_widget! { ProgressBar, ProgressBar, "progressbar" }

