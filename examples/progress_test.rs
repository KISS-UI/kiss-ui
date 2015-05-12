#[macro_use]
extern crate kiss_ui;

use kiss_ui::callback::OnShow;
use kiss_ui::container::Vertical;
use kiss_ui::dialog::Dialog;
use kiss_ui::progress::ProgressBar;
use kiss_ui::text::Label;
use kiss_ui::timer::Timer;

fn main() {
    kiss_ui::show_gui(|| {
        let regular = ProgressBar::new();
        let dashed = ProgressBar::new().set_dashed(true);

        let dialog = Dialog::new(
            Vertical::new(
                children![
                    Label::new("Regular:"),
                    regular.clone(),
                    Label::new("Dashed:"),
                    dashed.clone(),
                    Label::new("Indefinite:"),
                    ProgressBar::new().set_indefinite(true),
                ]
            )
        ).set_title("Progressbar Test");        

        dialog.set_on_show(move |_|{
            // Clone these so they can be moved into the inner closure
            let mut regular = regular.clone();
            let mut dashed = dashed.clone();

            let on_timer_interval = move |mut timer: Timer|{
                regular.add_value(0.1);
                dashed.add_value(0.1);

                if regular.get_value() == 1.0 {
                    timer.stop();
                }
            };

            Timer::new()
                .set_interval(1000)
                .set_on_interval(on_timer_interval)
                .start()             
        })
    });
}

