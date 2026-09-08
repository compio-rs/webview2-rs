use std::{env, fs, str};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("winmdgen/bin").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put WinUI 3 metadata in the bindgen/winmd dir");
    }

    println!("Generating WebView2 bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/webview2.txt"]);

    println!("Generating internal bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/internal.txt"]);

    println!("Done.");
    Ok(())
}
