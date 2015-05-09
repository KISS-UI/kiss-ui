extern crate kiss_ui;

use kiss_ui::button::Button;
use kiss_ui::callback::OnClick;
use kiss_ui::container::Vertical;
use kiss_ui::dialog::{self, Dialog};
use kiss_ui::text::{Label, TextBox};

fn main() {
    kiss_ui::show_gui(|| {
        Dialog::new(Vertical::new(|builder|{
            builder
                .add_child(Label::new("Enter a message:"))
                .add_child({
                    let mut textbox = TextBox::new()
                        .set_visible_columns(20);
                    textbox.set_name("my_textbox");
                    textbox
                })
                .add_child(
                    Button::new()
                        .set_label(Some("Save"))
                        .set_onclick(show_alert_message)
                );
        }))
        .set_title("Textbox Test")
    });
}

fn show_alert_message(clicked: Button) {
    let dialog = clicked.get_dialog().unwrap();
    let text_box = dialog.get_child("my_textbox").unwrap()
        .try_downcast::<TextBox>().ok().expect("child my_textbox was not a TextBox!");
    let text = text_box.get_text();

    dialog::popup_message_dialog("Message saved!", format!("Your message: {}", text));
}
