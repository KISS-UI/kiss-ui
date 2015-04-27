use super::BaseWidget;

pub struct Image(BaseWidget);

impl Image {
    pub fn new_grayscale(width: u32, height: u32, pixels: &[u8]) -> Image {
        assert!((width * height) as usize == pixels.len());
        let pixels_ptr = pixels.as_ptr() as *const i8;
        let ptr = unsafe { :iup_sys::IupImage(width as i32, height as i32, pixels_ptr) };
        Image(BaseWidget::from_ptr(ptr))
    }

    pub fn new_rgb(width: u32, height: u32, pixels: &[(u8, u8, u8)]) -> Image {
        assert!((width * height) as usize == pixels.len());
        let pixels_ptr = pixels.as_ptr() as *const i8;
        let ptr = unsafe { :iup_sys::IupImageRgb(width as i32, height as i32, pixels_ptr) };
        Image(BaseWidget::from_ptr(ptr))
    }

    pub fn new_rgba(width: u32, height: u32, pixels: &[(u8, u8, u8, u8)]) -> Image {
        assert!((width * height) as usize == pixels.len());
        let pixels_ptr = pixels.as_ptr() as *const i8;
        let ptr = unsafe { :iup_sys::IupImageRgba(width as i32, height as i32, pixels_ptr) };
        Image(BaseWiget::from_ptr(ptr))
    }
}
