#[macro_use]
extern crate kiss_ui;

use kiss_ui::container::Horizontal;
use kiss_ui::dialog::Dialog;
use kiss_ui::image::{Image, ImageContainer};
use kiss_ui::text::Label;

use std::iter;

fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;

    let col_range = || 0u8 .. 255;

    let image_data: Vec<_> = col_range()
        .flat_map(|val3| col_range().map(move |val2| (val2, val3)))
        .flat_map(|(val2, val3)| col_range().map(move |val1| (val1, val2, val3)))
        .take((WIDTH * HEIGHT) as usize)
        .collect();

    kiss_ui::show_gui(|| {
        Dialog::new(
            Horizontal::new(
                children![
                    Label::new("")
                        .set_image(Image::new_rgb(WIDTH, HEIGHT, &image_data)),
                ]
            )
        )
        .set_title("Image!")
    });
}
