fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new()
            .set_icon("assets/prog_icon.ico")
            .compile()
            .unwrap();
    }
}

