% Getting started with KISS-UI

KISS-UI has two main components, provided by the IUP library that powers it: an event loop, and callbacks. Most common GUI toolkits work this way. User code is mostly contained in callbacks, which are functions that the user sets to be invoked when the user performs an action in the UI, such as clicking a button or typing in a text box.

### The UI thread and long-running tasks

Since the KISS-UI event loop is single-threaded, it is important not to perform long-running tasks, such as reading files or performing network operations, in the same thread that the UI is running. Blocking in the so-called UI thread is bad because it stops input events from being processed and freezes the UI. This is usually what has happened when an application stops responding to input.

Most of the implementation details of KISS-UI, including the event loop, are not exposed to the user. It is simply important to be aware of the context in which your code is executing so you don't accidentally hang up the UI. For blocking or long-running tasks, such as I/O and processing, you should spawn a new thread. A simple asynchronous-task design that you can start with will be covered later in this guide.

### Getting KISS-UI running

KISS-UI is initialized with a single function, which initializes the underlying IUP library and starts the event loop running. Here, you can see how to import KISS-UI and call the initialization function, `show_gui()`, to show a basic window that tells the user, "Hello, world!":

```notest

#[macro_use]
extern crate kiss_ui;

// Common types and functions are exported in the `prelude` module
// for your convenience
use kiss_ui::prelude::*;

use kiss_ui::container::Horizontal;
use kiss_ui::text::Label;

fn main() {
    // The closure passed to this function is expected to return a `Dialog` instance 
	// describing the main window of your GUI and its contents.
    //
    // `Dialog` is exported in the`prelude` module because it is referenced often.
    kiss_ui::show_gui(||
		// No braces because this is all one expression!
		Dialog::new(
            Horizontal::new(
                // This is why `kiss_ui` is imported with `#[macro_use]`.
				//
				// This turns a list of disparate widget types into the same type
				// so they can be added to a container in a single `Vec`.
				//
				// This operation has no overhead besides creating the `Vec`
				// because of how widgets are implemented.
                children![
                    // A label is just that, a basic widget that renders static text.
                    Label::new("Hello, world!"),
                ]
            )
        )
        // Set the title of the window
        .set_title("Hello, world!")
        // And the size of the window
        .set_size_pixels(640, 480)
	);

	// Your UI is up and running!
}

```

If everything went right, you should see something like the following when you run the program:

On Linux:  
![](../../screenshots/GTK+/window_test.png)

On Windows:  
![](../../screenshots/Windows/window_test.png)

Note that the `kiss_ui::show_gui()` function will *not* return while the UI is running. If you want to do work while the UI is running, you should do that work in a new thread. The function will return after the last window is closed, so your program can do cleanup work in the main thread if need be.

Note also that Windows binaries need one extra step to enable the contemporary WIndows visual styles. Please see the README in the KISS-UI repository for more information.
