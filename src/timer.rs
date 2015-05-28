//! Timers that can invoke a callback on an interval.  
use widget_prelude::*;
use ::callback::Callback;

/// A timer that can invoke a callback on a configurable interval.
/// 
/// ##Note: Not a Renderable Widget
/// While this type can be dereferenced and converted to `BaseWidget`, it is *not* a renderable
/// widget and adding it to a container will have no visual effect.
///
/// ##Note: Resource Usage
/// This struct should be freed by calling `.destroy()` on it when it is no longer in use to free
/// any resources it has allocated. Otherwise, it will be freed when `kiss_ui::show_gui()` returns.
pub struct Timer(IUPPtr);

impl Timer {
    /// Create a new timer with a default interval.
    ///
    /// TODO: Document default interval.
    pub fn new() -> Timer {
        unsafe {
            let ptr = ::iup_sys::IupTimer();
            Self::from_ptr(ptr)
        }
    }

    /// Set the timer interval in milliseconds.
    pub fn set_interval(self, time: u32) -> Self {
        self.set_int_attribute(::attrs::TIME, time as i32);
        self
    }

    /// Set a callback to be invoked when the timer interval elapses.
    /// The callback will be invoked on every interval until `.stop()` is called.
    pub fn set_on_interval<Cb>(self, on_interval: Cb) -> Self where Cb: Callback<Self> {
       callback_impl! { ::attrs::ACTION_CB, self, on_interval, Timer }
       self
    }

    /// Start the timer. The callback will be invoked when the next interval elapses.
    pub fn start(self) -> Self {
        self.set_bool_attribute(::attrs::RUN, true);
        self
    }

    /// Stop the timer. The callback will not be invoked until the timer is restarted.
    pub fn stop(self) -> Self {
        self.set_bool_attribute(::attrs::RUN, false);
        self
    } 
}

impl_widget! { Timer, "timer" }

impl Destroy for Timer {}

