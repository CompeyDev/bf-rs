use windres::Build;

fn main() {
    if cfg!(target_os = "windows") {
        Build::new()
            .compile("assets/prog_meta.rc")
            .unwrap();
    }
}
