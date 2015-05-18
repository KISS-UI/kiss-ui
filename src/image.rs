//! Renderable image buffers.

use super::BaseWidget;

use std::ops::DerefMut;
use std::mem;

/// An image buffer allocated by IUP.
///
/// ##Note: Not a Renderable Widget
/// While this type can be dereferenced and converted to `BaseWidget`, it is *not* a renderable
/// widget and adding it to a container will have no visual effect.
///
/// Instead, it should be set on another widget type that implements the `ImageContainer` trait,
/// which will handle the actual rendering.
///
/// ##Note: Memory Usage
/// This struct should be freed by calling `.destroy()` on it when it is no longer in use.
/// Otherwise, it will be freed when `kiss_ui::show_gui()` exits^([citation needed]).
/// 
/// ##Note: Cloning
/// Cloning this image does not duplicate its allocation. Thus, destroying one image cloned from
/// another will destroy them both.
pub struct Image(BaseWidget);

impl Image {
    /// Create a new RGB image buffer from a slice of 3-byte tuples, copying the data into a new
    /// allocation.
    ///
    /// See `transmute_buffer_rgb()` in this module.
    ///
    /// ##Panics
    /// If `width * height` is not equal to `pixels.len()`.
    pub fn new_rgb(width: u32, height: u32, pixels: &[(u8, u8, u8)]) -> Image {
        assert_eq!((width * height) as usize, pixels.len());
        unsafe { 
            let ptr = ::iup_sys::IupImageRGB(width as i32, height as i32, pixels.as_ptr() as *const u8); 
            Image(BaseWidget::from_ptr(ptr))
        }
    }

    /// Create a new RGBA image buffer from a slice of 4-byte tuples, copying the data into a new
    /// allocation.
    ///
    /// See `transmute_buffer_rgba` in this module.
    ///
    /// ##Panics
    /// If `width * height` is not equal to `pixels.len()`.
    pub fn new_rgba(width: u32, height: u32, pixels: &[(u8, u8, u8, u8)]) -> Image {
        assert_eq!((width * height) as usize, pixels.len());
        unsafe { 
            let ptr = ::iup_sys::IupImageRGBA(width as i32, height as i32, pixels.as_ptr() as *const u8);
            Image(BaseWidget::from_ptr(ptr))
        }
    }

    /// Destroy this image, deallocating its backing memory. It will be removed from any elements
    /// it has been applied to.
    ///
    /// ##Warning
    /// Because they share backing allocations, this will affect all clones as well.
    pub fn destroy(self) {
        self.0.destroy()
    }
}

/// Cast a slice of bytes to a slice of 3-byte tuples without copying.
///
/// Returns `None` if `buf.len()` is not evenly divisible by 3.
pub fn transmute_buffer_rgb(buf: &[u8]) -> Option<&[(u8, u8, u8)]> {
    if buf.len() % 3 == 0 {
        Some(unsafe { mem::transmute(buf) })
    } else {
        None
    }
}

/// Cast a slice of bytes to a slice of 4-byte tuples without copying.
///
/// Returns `None` if `buf.len()` is not evenly divisible by 4.
pub fn transmute_buffer_rgba(buf: &[u8]) -> Option<&[(u8, u8, u8, u8)]> {
    if buf.len() % 4 == 0 {
        Some(unsafe { mem::transmute(buf) })
    } else {
        None
    }
}

impl_base_widget!{ Image, Image, "image" }

/// A trait describing an object that can render an image within itself.
pub trait ImageContainer: DerefMut<Target=BaseWidget> + Sized {
    /// Set the image this widget is to render and return `self` for method chaining.
    fn set_image(self, image: Image) -> Self {
        self.set_attr_handle(::attrs::IMAGE, image);
        self
    }

    /// Get a copy of the image set on this widget, if any.
    fn get_image(&self) -> Option<Image> {
        self.get_attr_handle(::attrs::IMAGE).map(Image)
    }
}

