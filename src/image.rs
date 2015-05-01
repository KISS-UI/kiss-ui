use super::BaseWidget;

pub struct Image(BaseWidget);

impl Image {
    pub fn new_rgb(width: u32, height: u32, pixels: &[(u8, u8, u8)]) -> Image {
        assert_eq!((width * height) as usize, pixels.len());
        let ptr = unsafe { ::iup_sys::IupImageRGB(width as i32, height as i32, pixels.as_ptr() as *const u8) };
        Image(BaseWidget::from_ptr(ptr))
    }

    pub fn new_rgba(width: u32, height: u32, pixels: &[(u8, u8, u8, u8)]) -> Image {
        assert_eq!((width * height) as usize, pixels.len());
        let ptr = unsafe { ::iup_sys::IupImageRGBA(width as i32, height as i32, pixels.as_ptr() as *const u8) };
        Image(BaseWidget::from_ptr(ptr))
    }
}

impl_base_widget!{ Image }
