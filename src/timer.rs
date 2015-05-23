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
pub struct Timer(BaseWidget);

impl Timer {
    /// Create a new timer with a default interval.
    ///
    /// TODO: Document default interval.
    pub fn new() -> Timer {
        unsafe {
            let ptr = ::iup_sys::IupTimer();
            Timer(BaseWidget::from_ptr(ptr))
        }
    }

    /// Set the timer interval in milliseconds.
    pub fn set_interval(mut self, time: u32) -> Self {
        self.set_int_attribute(::attrs::TIME, time as i32);
        self
    }

    /// Set a callback to be invoked when the timer interval elapses.
    /// The callback will be invoked on every interval until `.stop()` is called.
    pub fn set_on_interval<Cb>(mut self, on_interval: Cb) -> Self where Cb: Callback<Self> {
       callback_impl! { ::attrs::ACTION_CB, self, on_interval, Timer }
       self
    }

    /// Start the timer. The callback will be invoked when the next interval elapses.
    pub fn start(&mut self) {
        self.set_bool_attribute(::attrs::RUN, true);
    }

    /// Stop the timer. The callback will not be invoked until the timer is restarted.
    pub fn stop(&mut self) {
        self.set_bool_attribute(::attrs::RUN, false);
    }

    /// Destroy the timer, freeing its backing resources. If it was running, it will stop.
    ///
    /// ##Warning
    /// Because they share backing resources, this will affect all clones as well.
    pub fn destroy(self) {
        self.0.destroy()
    }
}

impl_base_widget! { Timer, Timer, "timer" }

