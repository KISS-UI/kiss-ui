//! Assorted types that can contain multiple widgets.
//!
//! All container types can be nested.
//!
//! Use the `children!{}` macro in this crate to convert a heterogeneous list of widgets into a
//! `Vec<BaseWidget>` for the container constructors.

use base::BaseWidget;
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

/// The behavior of this enum depends on its point of use.
#[derive(PartialEq, Eq, Copy, Clone)]
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


fn raw_handle_vec<B>(widgets: B) -> Vec<IUPPtr> where B: AsRef<[BaseWidget]> {
    let mut raw_handles: Vec<_> = widgets.as_ref().iter().cloned().map(BaseWidget::ptr).collect();
    raw_handles.push(::std::ptr::null_mut());
    raw_handles
}

/// A builder for `Absolute`, used to create and add children.
pub struct AbsoluteBuilder {
    handles: Vec<IUPPtr>,
}

impl AbsoluteBuilder {
    fn new() -> AbsoluteBuilder {
        AbsoluteBuilder { handles: Vec::new() }
    }

    /// Add a child to the `Absolute` at the given coordinates, relative to the top-left corner of
    /// the container.
    pub fn add_child_at<W: Widget>(&mut self, x: u32, y: u32, child: W) -> &mut Self {
        Absolute::set_child_pos(x, y, child);
        self.handles.push(child.ptr());
        self
    }
}


/// A container type that makes no effort to arrange its children. Instead, they must be positioned
/// manually.
pub struct Absolute(IUPPtr);

impl Absolute {
    /// Create a new absolute container using the given closure, which will be passed a mutable builder
    /// instance.
    ///
    /// The builder is necessary to ensure that the children have their positions set correctly, as
    /// `Widget::set_position` will not set the attributes that this particular container is
    /// expecting.
    pub fn new<F>(build_fn: F) -> Absolute where F: FnOnce(&mut AbsoluteBuilder) {
        let mut builder = AbsoluteBuilder::new();
        build_fn(&mut builder);

        unsafe {
            let ptr = ::iup_sys::IupCboxv(builder.handles.as_mut_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Set the position of a child of an `Absolute` relative to its top-left corner.
    pub fn set_child_pos<W: Widget>(x: u32, y: u32, child: W) {
        child.set_int_attribute(::attrs::CX, x as i32);
        child.set_int_attribute(::attrs::CY, y as i32);
    }
}

impl_widget! { Absolute, "cbox" }

/// A container widget that lines up its children from left to right.
pub struct Horizontal(IUPPtr);

impl Horizontal {
    /// Create a new horizontal container with the given vector or array of children, which may
    /// also be empty.
    ///
    /// See the `children![]` macro in this crate for more info.
    pub fn new<C>(children: C) -> Horizontal where C: AsRef<[BaseWidget]> {
        let mut raw_handles = raw_handle_vec(children);

        unsafe { 
            let ptr = ::iup_sys::IupHboxv(raw_handles.as_mut_ptr());
            Self::from_ptr(ptr)
        }
    }

    pub fn set_valign(self, valign: VAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_VERT, valign.as_cstr());
        self
    }

    pub fn set_elem_spacing_pixels(self, spacing: u32) -> Self {
        self.set_str_attribute(::attrs::GAP, spacing.to_string());
        self
    } 
}

impl_widget! { Horizontal, "hbox" }

/// A container widget that lines up its children from top to bottom.
pub struct Vertical(IUPPtr);

impl Vertical {
    pub fn new<C>(children: C) -> Vertical where C: AsRef<[BaseWidget]> {
       let mut raw_handles = raw_handle_vec(children); 

        unsafe {
            let ptr = ::iup_sys::IupVboxv(raw_handles.as_mut_ptr());
            Self::from_ptr(ptr)
        }
    }

    pub fn set_halign(self, halign: HAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_HORI, halign.as_cstr());
        self
    }

    pub fn set_elem_spacing_pixels(self, spacing: u32) -> Self {
        self.set_str_attribute(::attrs::GAP, spacing.to_string());
        self
    }
}


impl_widget! { Vertical, "vbox" }

/// A container widget that lines up its children from left to right, and from top to bottom.
pub struct Grid(IUPPtr);

impl Grid {
    pub fn new<C>(children: C) -> Grid where C: AsRef<[BaseWidget]> {
       let mut raw_handles = raw_handle_vec(children); 

        unsafe {
            let ptr = ::iup_sys::IupGridBoxv(raw_handles.as_mut_ptr());
            Self::from_ptr(ptr)
        }
    }
    pub fn set_valign(self, valign: VAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_VERT, valign.as_cstr());
        self
    }

    pub fn set_halign(self, halign: HAlign) -> Self {
        self.set_const_str_attribute(::attrs::ALIGNMENT_HORI, halign.as_cstr());
        self
    }

    /// Based on the orientation, set the number of children to place in a:
    ///
    /// * `Vertical`: **column**
    /// * `Horizontal`: **row**
    ///
    /// before beginning the next one.
    pub fn set_ndiv(self, ndiv: u32) -> Self {
        self.set_int_attribute(::attrs::NUMDIV, ndiv as i32);
        self
    }

    /// Sets how children are distributed in the container.
    ///
    /// * `Vertical`: The container will fill columns first.
    /// 
    /// Visual example (`ndiv=3` grid with 7 children):
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
    /// * `Horizontal`: The container will fill rows first. **Default.**
    ///
    /// Visual example (`ndiv=3` grid with 7 children):
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
    pub fn set_orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.set_const_str_attribute(::attrs::ORIENTATION, orientation.as_cstr());
        self
    }
}

impl_widget! { Grid, "matrix" }

/// Convert a heterogeneous list of widgets into a `Vec<BaseWidget>`,
/// suitable for passing to any function that takes `AsRef<[BaseWidget]>`, such as a constructor
/// for one of the container types.
#[macro_export]
macro_rules! children [
    // Accepts invocation with or without a final comma.
    ($($child:expr),+,) => (children![$($child),+]);
    ($($child:expr),+) => ({
        use ::kiss_ui::widget::Widget;
        vec![$($child.to_base()),+]
    });
    () => (vec![]);
];
