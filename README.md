KISS-UI [![Build Status](https://travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
=========
A UI framework for Rust based on the KISS (Keep It Simple, Stupid!) philosophy.

Powered by the [IUP][iup] GUI library for C by Tecgraf, via the bindings created for [iup-rust][iup-rust].

(No relation to the equally awesome [kiss3d][kiss3d].)

####KISS-UI builds on all Rust release channels!

[kiss3d]: https://github.com/sebcrozet/kiss3d
[iup]: http://webserver2.tecgraf.puc-rio.br/iup/
[iup-rust]: https://github.com/dcampbell24/iup-rust

Contents
--------

* [Documentation](#documentation)
* [Usage](#usage)
* [Installing IUP Binaries](#installing-iup-binaries)
  * [Windows](#windows)
  * [Linux](#linux)
  * [OS X](#os-x)
* [Comparison to Other UI Frameworks](#comparison-to-other-ui-frameworks)
* [Enabling Visual Styles on Windows](#enabling-visual-styles-on-windows)

Documentation
-------------
[User Guide (Getting Up and Running)](http://cybergeek94.github.io/kiss-ui/docs/guide)
[API Documentation](http://cybergeek94.github.io/kiss-ui/docs/api)

Comparison to Other UI Frameworks
---------------------------------
**NOTE**: This list is *far* from exhaustive and may contain outdated information.

Pull requests for corrections and additions are welcome!

* KISS-UI
  * Build Status: [![Build Status](https://travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
  * Supported Platforms: Windows (using Win32 APIs), Linux and Mac (using GTK+)
  * Native Look and Feel: **Yes**
  * "Hello, World!" LOC: **[18][kiss-ui-hw]**
  * External Crates: **2**
  * External Native Libs: 1
* [PistonDevelopers/conrod][conrod]
  * Build Status: [![Build Status](https://travis-ci.org/PistonDevelopers/conrod.svg?branch=master)](https://travis-ci.org/PistonDevelopers/conrod)
  * Supported Platforms: Windows, Mac, Linux
  * Native Look and Feel: No
  * "Hello, World!" LOC: [40][conrod-hw] (estimated based on linked example)
  * External Crates: 9 (not including testing crates and transitive dependencies)
  * External Native Libs: **~0** (depends on backend used)
* [rust-gnome/gtk][rgtk]
  * Build Status: [![Build Status](https://travis-ci.org/rust-gnome/gtk.png?branch=master)](https://travis-ci.org/rust-gnome/gtk)
  * Supported Platforms: Windows, Mac, Linux
  * Native Look and Feel: **Yes**
  * "Hello, World!" LOC: [23][rust-gnome-hw]
  * External Crates: 10 (1 local but pulled from Crates.io)
  * External Native Libs: ~5 (installed on most Linux distros/external on Windows, Mac)

Lines of code should be listed based on the `# sloc` stat on the Github file page. The raw linecount includes empty lines, which can arbitrarily affect the linecount.

Enabling Visual Styles on Windows
---------------------------------
Since Rust/Cargo currently do not support adding resource items to executables, Windows XP and later need an external manifest file to enable visual styles in KISS-UI applications. Otherwise the visual style will be Windows Classic.

However, we have made this very simple to do! Simply copy the `kiss-app.manifest` file from this repo into the folder of your KISS-UI based executable, rename the file to `<executable name>.manifest` (including the `.exe` extension, e.g. `my_executable.exe.manifest`), and run the executable as-is. You may need to delete and replace or rebuild the executable for this to take effect, as Windows appears to cache manifest file data, likely to avoid reparsing it on each run.

Optionally, you can edit the `name=` and the `<description>` values in the manifest file, using any text editor. However, it is unclear to the author what these actually affect.
  
[kiss-ui-hw]: https://github.com/cybergeek94/kiss-ui/blob/master/examples/window_test.rs

[conrod]: https://github.com/PistonDevelopers/conrod
[conrod-hw]: https://github.com/PistonDevelopers/conrod/blob/master/examples/counter.rs

[rust-gnome-hw]: https://github.com/rust-gnome/examples/blob/master/src/basic.rs

[rgtk]: https://github.com/rust-gnome/gtk


