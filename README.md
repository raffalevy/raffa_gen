My attempt to make fast-rendering generative art in Rust. Also just a playground project for me to learn about 2D graphics. Currently only runs on MacOS using a Quartz backend.

raffa_gen makes use of the bindings to Cocoa, Core Foundation, and Cairo provided by the Servo Project and gtk-rs in order to call into these librarys from Rust.

Much of the drawing is performed on a raw image buffer inside of a Cairo image surface.

Will probably look weird on non-Retina displays.

# Building
To build and run the project, run the following command in the project directory:

    sh run.sh

The build script starts by compiling `src/setup.m`, the Objective-C file which sets up the Quartz environment, using Clang. It is then archived into the static library `libsetup.a`. The bulk of the project, written in Rust, is built using Cargo. Finally, the script runs the executable, which is symlinked into `Rustt.app`. (Bundling the executable into a `.app` seems to be necessary to get the menu bar to work.)