use super::BaseWidget;

use std::ops::DerefMut;

pub struct Image(BaseWidget);

impl Image {
    pub fn new_rgb(width: u32, height: u32, pixels: &[(u8, u8, u8)]) -> Image {
        assert_eq!((width * height) as usize, pixels.len());
        unsafe { 
            let ptr = ::iup_sys::IupImageRGB(width as i32, height as i32, pixels.as_ptr() as *const u8); 
            Image(BaseWidget::from_ptr(ptr))
        }
    }

    pub fn new_rgba(width: u32, height: u32, pixels: &[(u8, u8, u8, u8)]) -> Image {
        assert_eq!((width * height) as usize, pixels.len());
        unsafe { 
            let ptr = ::iup_sys::IupImageRGBA(width as i32, height as i32, pixels.as_ptr() as *const u8);
            Image(BaseWidget::from_ptr(ptr))
        }
    }
}

impl_base_widget!{ Image, Image, "image" }

pub trait ImageContainer: DerefMut<Target=BaseWidget> + Sized {
    fn set_image(self, image: Image) -> Self {
        // Deallocate the existing image if there is one.
        self.get_attr_handle(::attrs::IMAGE).map(|img| img.destroy());
        self.set_attr_handle(::attrs::IMAGE, image);

        self
    }

    fn get_image(&self) -> Option<Image> {
        self.get_attr_handle(::attrs::IMAGE).map(Image)
    }
}

