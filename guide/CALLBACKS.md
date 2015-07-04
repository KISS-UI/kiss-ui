% Using Callbacks Effectively

Callbacks are a major component of KISS-UI, and for that matter, most event-based GUI toolkits. This includes Windows Forms, Windows Presentation Foundation, GTK+, Java's Swing, OS X's Cocoa, Android's UI framework, and others. GTK+ calls them "signals".

Callbacks are invoked when user input, such as a mouse click or a keystroke, or a system event, such as a timer reaching a set interval, triggers them.

KISS-UI callbacks are plain-old Rust closures, specifically `FnMut` closures. There is also the `Callback` trait you can implement directly on your own structs if you'd prefer.

Specifically, the bound on callback closures are `FnMut + 'static`, which means they have to take ownership of their captured variables (called "upvars") and these values cannot contain non-`'static` references. Capturing upvars by-value is indicated to the compiler by means of the `move` keyword placed before the closure bars: `let closure = move |args| { statement(args); }`

Since callback closures are `FnMut`, you can perform mutation on captured upvars! This could be very useful, for example, to keep a running total of how many times a button has been pressed just by capturing a mutable integer binding in its `.set_on_click()` callback.

In the following example, we move both an integer and a `Label` widget into a `Button` widget's `OnClick` callback, so we can mutate the integer and update the text on the label:

```notest
#[macro_use]
extern crate kiss_ui;

use kiss_ui::prelude::*;

use kiss_ui::button::Button;
use kiss_ui::text::Label;


fn main() {
	kiss_ui::show_gui(|| {
        // We create the variable out here so that the callback knows it needs to capture it.
        // If we created it inside the callback it would be reinitialized to 0 every time.
        let mut count = 0;
        let label = Label::new("Button presses: 0");

        Dialog::new(
            Vertical::new(children! [
                label,
                Button::new("Press me!").set_on_click(move |button| {
                    count += 1;
                    label.set_text(format!("Button presses: {}", count));
                })
            ])
        )  
    });     
}
```

####Sharing data between closures

If you need to access the same values from multiple closures, you should place said values in `Rc`, and pass a clone to each closure. You can use `RefCell` or `Cell` if you need to be able to mutate them.

Additionally, you might find the `thread_local!{}` macro useful. It allows you to store data statically but each thread has its own independent view of the data so the stored type doesn't need to be `Sync`. Have a look in the Rust API documentation for more info, or you can see how KISS-UI uses it in [`lib.rs`](../../src/lib.rs) and [`callbacks.rs`](../../src/callbacks.rs).
	
