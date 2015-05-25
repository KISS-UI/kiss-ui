#[macro_use]
extern crate kiss_ui;

use kiss_ui::button::Button;
use kiss_ui::container::Horizontal;
use kiss_ui::callback::{OnClick, CallbackStatus};
use kiss_ui::dialog::{self, AlertPopupBuilder, Dialog};
use kiss_ui::text::Label;

fn main() {
    kiss_ui::show_gui(||
        Dialog::new(
            Horizontal::new(
                children![               
                    Button::new()
                        .set_label(Some("Message"))
                        .set_onclick(show_message_dialog),
                    Button::new()
                        .set_label(Some("Alert"))
                        .set_onclick(show_alert_dialog),
                    Button::new()
                        .set_label(Some("Close"))
                        .set_onclick(close_dialog),
                ]
            )
            .set_elem_spacing_pixels(10)                   
        )
        .set_title("Button test!")
    );
}

fn show_message_dialog(_: Button) {
    dialog::message_popup("Good job!", "You clicked the button!");
}

fn show_alert_dialog(_: Button) {
    let res = AlertPopupBuilder::new("Alert!", "You clicked the other button!", "Yes")
        .button2("No")
        .button3("Cancel")
        .popup();

    println!("Alert result = {}", res);
}

fn close_dialog(_: Button) -> CallbackStatus {
    println!("Closing dialog!");
    CallbackStatus::Close
}
