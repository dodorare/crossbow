pub const MINIMAL_BEVY_CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
version = "0.1.0"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }

[target.'cfg(target_os = "android")'.dependencies]
android-activity = { version = "0.6.1", features = ["native-activity"] }
"#;

pub const BEVY_LIB_RS_VALUE: &str = r#"pub fn main() { println!("hello"); }

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(_app: android_activity::AndroidApp) { main(); }
"#;

pub const BEVY_MAIN_RS_VALUE: &str = "fn main() { example::main(); }\n";

pub const MINIMAL_MQ_CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
version = "0.1.0"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }
anyhow = "1.0"
macroquad = "0.4.16"

[package.metadata.android]
runtime = "miniquad"
"#;

pub const MQ_MAIN_RS_VALUE: &str = r#"
#[macroquad::main("Macroquad 3D")]
pub async fn main() -> anyhow::Result<()> {Ok(())}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "C" fn quad_main() { main(); }
"#;

pub const MQ_BIN_RS_VALUE: &str = "fn main() { example::main(); }\n";

pub const STRINGS_XML_VALUE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="hello">Hello!</string>
</resources>
"#;
