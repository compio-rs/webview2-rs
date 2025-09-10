use std::{env, fs, str};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("winmdgen/bin").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put WinUI 3 metadata in the bindgen/winmd dir");
    }

    println!("Generating WebView2 bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/webview2.txt"]).unwrap();

    println!("Patching link commands...");
    patch_link();

    println!("Done.");
    Ok(())
}

fn patch_link() {
    const BINDINGS_MOD: &str = "webview2/src/bindings.rs";

    let contents = fs::read_to_string(BINDINGS_MOD).expect("failed to read bindings.rs");
    let contents = contents.replace(r#"windows_link::link!"#, r#"crate::link!"#);
    fs::write(BINDINGS_MOD, &contents).expect("failed to write bindings.rs");
}
