# xkbcommon-dl

[![](http://meritbadge.herokuapp.com/xkbcommon-dl)](https://crates.io/crates/xkbcommon-dl)
[![Docs.rs](https://docs.rs/xkbcommon-dl/badge.svg)](https://docs.rs/xkbcommon-dl)

xkbcommon and xkbcommon-x11 Rust bindings. Can be dynamically loaded at runtime or linked against at compile time.

By default will be dynamically loaded. Unset the `dlopen-xkbcommon` feature to link at compile time.

lib.rs is an almost verbatim copy of [wayland-kbd's xkbcommon module](https://github.com/Smithay/wayland-kbd/blob/master/src/ffi/mod.rs), along with [update-keysyms.sh](https://github.com/Smithay/wayland-kbd/blob/master/update-keysyms.sh).
