fn main() {
    if cfg!(target_os = "windows") {
        windres::Build::new()
            .compile("assets/prog_meta.rc")
            .unwrap();
    }
}
