extern crate kiss_ui;

use kiss_ui::BaseWidget;
use kiss_ui::callback::OnShow;
use kiss_ui::container::Vertical;
use kiss_ui::dialog::Dialog;
use kiss_ui::progress::ProgressBar;
use kiss_ui::text::Label;
use kiss_ui::timer::Timer;

const REGULAR: &'static str = "regular_progressbar";
const DASHED: &'static str = "dashed_progressbar";

fn main() {
    kiss_ui::show_gui(|| {
        Dialog::new(Vertical::new(|builder|{
            builder
                .add_child(Label::new("Regular:"))
                .add_child({
                    let regular = ProgressBar::new();
                    regular.store(REGULAR);
                    regular                
                })
                .add_child(Label::new("Dashed:"))
                .add_child({
                    let dashed = ProgressBar::new().set_dashed(true);
                    dashed.store(DASHED);
                    dashed
                })
                .add_child(Label::new("Marquee:"))
                .add_child(ProgressBar::new().set_infinite(true));
        }))
        .set_title("Progressbar Test")
        .set_on_show(on_show_dialog)
    });
}

fn on_show_dialog(_: Dialog) {
    Timer::new()
        .set_interval(1000)
        .set_on_interval(on_timer_interval)
        .start();
}    

fn on_timer_interval(timer: Timer) {
    let mut regular = BaseWidget::load(REGULAR).unwrap()
        .try_downcast::<ProgressBar>().ok().unwrap();

    let mut dashed = BaseWidget::load(DASHED).unwrap()
        .try_downcast::<ProgressBar>().ok().unwrap();

    regular.add_value(0.1);
    dashed.add_value(0.1);

    if regular.get_value() == 1.0 {
        timer.stop();
    }
}
