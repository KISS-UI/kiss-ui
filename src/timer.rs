use ::BaseWidget;
use ::callback::Callback;

pub struct Timer(BaseWidget);

impl Timer {
    pub fn new() -> Timer {
        unsafe {
            let ptr = ::iup_sys::IupTimer();
            Timer(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_interval(mut self, time: u32) -> Self {
        self.set_int_attribute(::attrs::TIME, time as i32);
        self
    }

    pub fn set_on_interval<Cb>(mut self, on_interval: Cb) -> Self where Cb: Callback<(Self,)> {
       callback_impl! { ::attrs::ACTION_CB, self, on_interval, Timer }
       self
    }

    pub fn start(mut self) {
        self.set_bool_attribute(::attrs::RUN, true);
    }

    pub fn stop(mut self) {
        self.set_bool_attribute(::attrs::RUN, false);
    }
}

impl_base_widget! { Timer, Timer, "timer" }

