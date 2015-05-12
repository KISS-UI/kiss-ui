//! Assorted types that can contain multiple widgets.
//!
//! All container types can be nested.

use super::BaseWidget;

/// Vertical alignment setting, used by `Horizontal` and `Grid`.
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

/// Horizontal alignment setting, used by `Vertical` and `Grid`.
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

fn raw_handle_vec<B>(widgets: B) -> Vec<*mut ::iup_sys::Ihandle> where B: AsRef<[BaseWidget]> {
    let mut raw_handles: Vec<_> = widgets.as_ref().iter().map(|child| child.0).collect();
    raw_handles.push(::std::ptr::null_mut());
    raw_handles
}

/// A container type that makes no effort to arrange its children. Instead, they must be positioned
/// manually.
pub struct Absolute(BaseWidget);

/// A container widget that lines up its children from left to right.
pub struct Horizontal(BaseWidget);

impl Horizontal {
    pub fn new<C>(children: C) -> Horizontal where C: AsRef<[BaseWidget]> {
        let mut raw_handles = raw_handle_vec(children);

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

/// A container widget that lines up its children from top to bottom.
pub struct Vertical(BaseWidget);

impl Vertical {
    pub fn new<C>(children: C) -> Vertical where C: AsRef<[BaseWidget]> {
       let mut raw_handles = raw_handle_vec(children); 

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

/// A container widget that lines up its children from left to right, and from top to bottom.
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

/// Convert a list of widgets into an array of `BaseWidget`.
/// Suitable for passing to any function that takes `AsRef<[BaseWidget]>`.
///
/// ##Note
/// If you are getting an error saying `[BaseWidget; <integer>] does not implement
/// AsRef<[BaseWidget]>`, then this array is too large (`AsRef<[T]>` is only implemented for
/// arrays up to size `[T; 32]`). To fix this, call `.to_owned()` on the result to convert it to
/// `Vec<BaseWidget>`, which will work for any size. 
#[macro_export]
macro_rules! children [
    ($($child:expr),+,) => ([$($child.into()),+]);
    () => ([]);
];
