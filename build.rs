use std::env;

fn main() {
    let x11 = env::var_os("CARGO_FEATURE_X11").is_some();
    let dlopen = env::var_os("CARGO_FEATURE_DLOPEN_XKBCOMMON").is_some();
    let statik = env::var_os("CARGO_FEATURE_STATIC").is_some();

    if !dlopen {
        pkg_config::Config::new().statik(statik).probe("xkbcommon").unwrap();
        if x11 {
            pkg_config::Config::new().statik(statik).probe("xkbcommon-x11").unwrap();
            pkg_config::Config::new().statik(statik).probe("xcb-xkb").unwrap();
        }
    }
}

