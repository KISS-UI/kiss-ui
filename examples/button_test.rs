extern crate kiss_ui;

use kiss_ui::button::Button;
use kiss_ui::container::Horizontal;
use kiss_ui::callback::OnClick;
use kiss_ui::dialog::Dialog;
use kiss_ui::text::Label;

fn main() {
    kiss_ui::show_gui(||
        Dialog::new(Horizontal::new(|builder|{
            builder.add_child(
                Button::new()
                    .set_label(Some("Click me!"))
                    .set_onclick(show_new_dialog) 
            );
        }))
        .set_title("Button test!")
        .set_size_pixels(280, 140)
    );
}

fn show_new_dialog(_: Button) {
    println!("Button clicked!");

    Dialog::new(Label::new("You clicked the button!"))
        .set_title("Button clicked!")
        .set_size_pixels(180, 90)
        .show();
}
