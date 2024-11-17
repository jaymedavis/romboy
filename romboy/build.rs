use std::env;

extern crate winresource;

fn main() {
    slint_build::compile("ui/main-window.slint").unwrap();

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        winresource::WindowsResource::new()
            .set_icon("assets/logo.ico")
            .compile().unwrap();
    }
}
