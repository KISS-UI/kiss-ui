KISS-UI [![Build Status](https://travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
=========
A UI framework for Rust based on the KISS (Keep It Simple, Stupid!) philosophy.

Powered by the [IUP][iup] GUI library for C by Tecgraf, via the bindings created for [iup-rust][iup-rust].

(No relation to the equally awesome [kiss3d][kiss3d].)

[kiss3d]: https://github.com/sebcrozet/kiss3d
[iup]: http://webserver2.tecgraf.puc-rio.br/iup/
[iup-rust]: https://github.com/dcampbell24/iup-rust

Contents
--------

* [Documentation](#documentation)
* [Usage](#usage)
* [Installing IUP Binaries](#installing-iup-binaries)
* [Comparison to Other UI Frameworks](#comparison-to-other-ui-frameworks)
* [Enabling Visual Styles on Windows](#enabling-visual-styles-on-windows)

Documentation
-------------
[`kiss-ui` docs hosted on Rust-CI](http://rust-ci.org/cybergeek94/kiss-ui/doc/kiss_ui/)

Usage
-----

Simply add the following to your `Cargo.toml`:

```
[dependencies.kiss-ui]
git = "https://github.com/cybergeek94/kiss-ui
```

####KISS-UI builds on all Rust release channels!

[iup-dl]: http://sourceforge.net/projects/iup/files/3.14/

Installing IUP Binaries
-------------------

You will need to install the IUP binaries for your system, which are available for download [here][iup-dl]. 

Consult the following for which files to download and where to install them. The specific steps depend on your platform and preferred method of linking: dynamic or static.

PRs amending or adding instructions for any platform are very welcome.

***
###Windows
####Dynamic linking
* Navigate to `Windows Libraries/Dynamic`
  * 32-bit: Download `iup-3.14_Win32_dllw4_lib.zip`
  * 64-bit: Download `iup-3.14_Win64_dllw4_lib.zip`
* Extract all `.dll` files to a folder where the linker can find them (pick one):
  * `<Rust install>/bin/rustlib/<platform target>/lib/` (recommended)
  * (using MinGW/MSYS) `<MinGW/MSYS install>/usr/lib`
* Copy the same DLLs to a folder in your PATH (pick one):
  * `<Rust install>/bin/` (recommended)
  * Create a folder anywhere and add it to your PATH.

You should **NEVER** place arbitrary files in your Windows install folder, no matter how benign.

####Static Linking
TODO
***
###Linux
TODO (Some distros may have IUP binaries in their package managers, but the author is currently unsure.)
***
###OS X
IUP does not currently have binaries available for OS X. However, since it is powered by GTK+ on Linux, it should be possible to build it for OS X using (mostly) the same steps. Feel free to try it and let us know how it goes.
***

Comparison to Other UI Frameworks
---------------------------------
**NOTE**: This list is *far* from exhaustive and may contain outdated information.

Pull requests for corrections and additions are welcome!

* KISS-UI
  * Build Status: [![Build Status](https://travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
  * Supported Platforms: Windows (using Win32 APIs), Linux (using GTK+), Mac (theoretically)
  * Native Look and Feel: **Yes**
  * "Hello, World!" LOC: **[18][kiss-ui-hw]**
  * External Crates: **2** (includiing 1 local)
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
  * "Hello, World!" LOC: ?
  * External Crates: 10 (1 local but pulled from Crates.io)
  * External Native Libs: ~5 (installed on most Linux distros/external on Windows, Mac)

Enabling Visual Styles on Windows
---------------------------------
Since Rust/Cargo currently do not support adding resource items to executables, Windows XP and later need an external manifest file to enable visual styles in KISS-UI applications. Otherwise the visual style will be Windows Classic.

However, we have made this very simple to do! Simply copy the `kiss-app.manifest` file from this repo into the folder of your KISS-UI based executable, rename the file to `<executable name>.manifest` (including the `.exe` extension, e.g. `my_executable.exe.manifest`), and run the executable as-is. You may need to delete and replace or rebuild the executable for this to take effect, as Windows appears to cache manifest file data, likely to avoid reparsing it on each run.

Optionally, you can edit the `name=` and the `<description>` values in the manifest file, using any text editor. However, it is unclear to the author what these actually affect.
  
[kiss-ui-hw]: https://github.com/cybergeek94/kiss-ui/blob/master/examples/window_test.rs

[conrod]: https://github.com/PistonDevelopers/conrod
[conrod-hw]: https://github.com/PistonDevelopers/conrod/blob/master/examples/counter.rs

[rgtk]: https://github.com/rust-gnome/gtk


