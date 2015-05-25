#[macro_use]
extern crate kiss_ui;

use kiss_ui::dialog::Dialog;
use kiss_ui::utils::move_cell::MoveCell;


fn main() {
    kiss_ui::show_gui(||
        Dialog::empty()
            .set_title("Okay context!")
            .set_size_pixels(210, 70)
    );

    println!("Doing bad things!");

    // Apparently IUP knows when it's not running and it doesn't do anything bad.

    Dialog::empty()
        .set_title("Bad context!")
        .set_size_pixels(210, 70)
        .show();
    
    println!("Apparently not!");
    
    kiss_ui::show_gui(||
        Dialog::empty()
            .set_title("Okay context!")
            .set_size_pixels(210, 70)
    );
}

