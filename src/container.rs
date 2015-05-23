//! Assorted types that can contain multiple widgets.
//!
//! All container types can be nested.

use widget_prelude::*;

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

#[derive(Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {
    #[doc(hidden)]
    pub fn as_cstr(self) -> &'static str {
        use self::Orientation::*;

        match self {
            Vertical => cstr!("VERTICAL"),
            Horizontal => cstr!("HORIZONTAL"),
        }
    }
}


fn raw_handle_vec<B>(widgets: B) -> Vec<*mut ::iup_sys::Ihandle> where B: AsRef<[BaseWidget]> {
    let mut raw_handles: Vec<_> = widgets.as_ref().iter().map(BaseWidget::ptr).collect();
    raw_handles.push(::std::ptr::null_mut());
    raw_handles
}

/// A container type that makes no effort to arrange its children. Instead, they must be positioned
/// manually.
pub struct Absolute(BaseWidget);

/// A container widget that lines up its children from left to right.
pub struct Horizontal(BaseWidget);

impl Horizontal {
    /// Create a new horizontal container with the given vector or array of children, which may
    /// also be empty.
    ///
    /// See the `children![]` macro in this crate for more info.
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

    /// Sets how children are distributed in the container.
    ///
    /// * `Vertical`: The container will prefer full columns to rows.
    /// 
    /// Visual example (3x3 grid with 7 children):
    /// <table>
    ///     <tr>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///     </tr>
    ///     <tr>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///     </tr>
    ///     <tr>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///     </tr>
    /// </table>
    ///
    /// * `Horizontal`: The container will prefer full rows to columns.
    ///
    /// Visual example (3x3 grid with 7 children):
    /// <table>
    ///     <tr>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///     </tr>
    ///     <tr>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///         <td>Child</td>
    ///     </tr>
    ///     <tr>
    ///         <td>Child</td>
    ///     </tr>
    /// </table>
    ///
    pub fn set_orientation(mut self, orientation: Orientation) -> Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, orientation.as_cstr());
        self
    }
}

impl_base_widget! { Grid, Grid, "matrix" }

/// Convert a heterogeneous list of widgets into an array of `BaseWidget`,
/// suitable for passing to any function that takes `AsRef<[BaseWidget]>`.
///
/// ##Note
/// If you are getting an error saying `[BaseWidget; <integer>] does not implement
/// AsRef<[BaseWidget]>`, then this array is too large. This is because `AsRef<[T]>` is only implemented for
/// arrays up to size 32. To fix this, use `children_vec!` instead, which will work for any size. 
#[macro_export]
macro_rules! children [
    ($($child:expr),+,) => ([$($child.into()),+]);
    () => ([]);
];

/// Convert a heterogeneous list of widgets into a vector of `BaseWidget`,
/// suitable for passing to any function that takes `AsRef<[BaseWidget]>`.
///
/// ##Note
/// While this may be used for any number of children, you should prefer the `children![]` macro,
/// as it uses a stack-allocated array instead of a heap-allocated vector. Use this macro only for
/// child lists of size 33 or more, as `AsRef<[T]>` is only implemented for arrays of up to
/// size 32.
#[macro_export]
macro_rules! children_vec [
    ($($child:expr),+) => (vec![$($child.into()),+]);
    () => (vec![]);
];
