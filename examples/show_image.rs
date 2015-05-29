#[macro_use]
extern crate kiss_ui;

use kiss_ui::container::Horizontal;
use kiss_ui::dialog::Dialog;
use kiss_ui::image::{Image, ImageContainer};
use kiss_ui::text::Label;

fn main() {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;

    let image_data: Vec<_> = (0..HEIGHT)
        .flat_map(|y| (0..WIDTH).map(move |x| color(x, y)))
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
        .set_title(format!("Image! ({width} x {height})", width=WIDTH, height=HEIGHT))
    });
}

/// Play with this function and let us know what you come up with!
fn color(x: u32, y: u32) -> (u8, u8, u8) {
    // Suggested by /u/GBGamer117
    ((x ^ y) as u8, y as u8, x as u8)
    
    // Suggested by /u/Effnote
    // ((x ^ y) as u8, ((x + 2) ^ (y + 1)) as u8, ((x + 4) ^ (y + 2)) as u8)  

    // Suggested by /u/ImSoCabbage
    // let val = (x ^ y) as u8;
    // (val, val, val) 
    // (255 - val, val, val) // Inverted red 
}

