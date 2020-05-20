# flutterbug

A basic set of X11 bindings. This is intended to be used by Beetle as a comprehensive, safe interface to the X Window System. Of note is that this interface uses Xlib instead of XCB, because:

1). I know Xlib better than XCB.

2). I couldn't find a good set of Rust bindings to XCB.

If you want, you can use flutterbug instead of Beetle, if you want a direct interface to X11. Personally, I wouldn't recommend it; if compiled for Linux, Beetle should just be a thin layer on top of flutterbug. See the `examples` directory for usage.
