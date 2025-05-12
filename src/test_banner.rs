use rascii_art::{RenderOptions, render_to};
use std::path::PathBuf;

pub fn banner() {
    let mut buffer = String::new(); // ← here’s the buffer

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("images/banner.png");

    render_to(
        path.to_str().unwrap(), // absolute-safe path
        &mut buffer,            // ← buffer used here
        &RenderOptions::new()
            .width(45)
            .height(25)
            .colored(true)
            .charset(&[".", "-", "$", "#"]),
    )
    .unwrap();

    println!("{}", buffer); // ← render result printed
}
