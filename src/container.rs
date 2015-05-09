use super::BaseWidget;

#[derive(Copy, Clone)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

impl VAlign {
    fn as_cstr(self) -> &'static str {
        use self::VAlign::*;

        match self {
            Top => cstr!("ATOP"),
            Center => cstr!("ACENTER"),
            Bottom => cstr!("ABOTTOM"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl HAlign {
    fn as_cstr(self) -> &'static str {
        use self::HAlign::*;

        match self {
            Left => cstr!("ALEFT"),
            Center => cstr!("ACENTER"),
            Right => cstr!("ARIGHT"),
        }
    }
}

#[derive(Default)]
pub struct ContainerBuilder(Vec<BaseWidget>);

impl ContainerBuilder {
    pub fn add_child<W: Into<BaseWidget>>(&mut self, child: W) -> &mut Self {
        self.0.push(child.into());
        self
    }

    fn to_raw_handles(self) -> Vec<*mut ::iup_sys::Ihandle> {
        self.0.map_in_place(|val| val.0)
    }
}

pub struct Absolute(BaseWidget);

pub struct Horizontal(BaseWidget);

impl Horizontal {
    pub fn new<F>(build_fn: F) -> Horizontal where F: FnOnce(&mut ContainerBuilder) {
        let mut builder = ContainerBuilder::default();
        build_fn(&mut builder);
        builder.add_child(unsafe { BaseWidget::null() });

        let mut raw_handles = builder.to_raw_handles();
        unsafe { 
            let ptr = ::iup_sys::IupHboxv(raw_handles.as_mut_ptr());
            Horizontal(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_valign(mut self, valign: VAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_VERT, valign.as_cstr());
        self
    }

    pub fn set_elem_spacing_pixels(mut self, spacing: u32) -> Self {
        self.set_str_attribute(::attrs::GAP, spacing.to_string());
        self
    } 
}

impl_base_widget! { Horizontal, Horizontal, "hbox" }

pub struct Vertical(BaseWidget);

impl Vertical {
    pub fn new<F>(build_fn: F) -> Vertical where F: FnOnce(&mut ContainerBuilder) {
        let mut builder = ContainerBuilder::default();
        build_fn(&mut builder);
        builder.add_child(unsafe { BaseWidget::null() });

        let mut raw_handles = builder.to_raw_handles();

        unsafe {
            let ptr = ::iup_sys::IupVboxv(raw_handles.as_mut_ptr());
            Vertical(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn set_halign(mut self, halign: HAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_HORI, halign.as_cstr());
        self
    }

    pub fn set_elem_spacing_pixels(mut self, spacing: u32) -> Self {
        self.set_str_attribute(::attrs::GAP, spacing.to_string());
        self
    }
}



impl_base_widget! { Vertical, Vertical, "vbox" }

pub struct Grid(BaseWidget);

impl Grid {
    pub fn set_valign(mut self, valign: VAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_VERT, valign.as_cstr());
        self
    }

    pub fn set_halign(mut self, halign: HAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_HORI, halign.as_cstr());
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
}

impl_base_widget! { Grid, Grid, "matrix" }
