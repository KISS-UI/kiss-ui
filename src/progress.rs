use ::BaseWidget;

pub struct ProgressBar(BaseWidget);

impl ProgressBar {
    pub fn new() -> ProgressBar {
        unsafe {
            let ptr = ::iup_sys::IupProgressBar();
            ProgressBar(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_infinite(mut self, is_infinite: bool) -> Self {
        self.set_bool_attribute(::attrs::MARQUEE, is_infinite);
        self
    }

    pub fn set_dashed(mut self, dashed: bool) -> Self {
        self.set_bool_attribute(::attrs::DASHED, dashed);
        self
    }

    pub fn set_max(mut self, max: f32) -> Self {
        self.set_float_attribute(::attrs::MAX, max);
        self
    }

    pub fn set_min(mut self, min: f32) -> Self {
        self.set_float_attribute(::attrs::MIN, min);
        self
    }

    pub fn set_vertical(mut self) -> Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, ::attrs::values::VERTICAL);
        self
    }

    pub fn set_horizontal(mut self) -> Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, ::attrs::values::HORIZONTAL);
        self
    }

    pub fn set_value(&mut self, val: f32) {
        self.set_float_attribute(::attrs::VALUE, val);        
    }
    
    pub fn get_value(&self) -> f32 {
        self.get_float_attribute(::attrs::VALUE)
    }

    pub fn add_value(&mut self, amt: f32) {
        let val = self.get_float_attribute(::attrs::VALUE);
        self.set_float_attribute(::attrs::VALUE, val + amt);
    }
}

impl_base_widget! { ProgressBar, ProgressBar, "progressbar" }

