pub const MINIMAL_BEVY_CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
version = "0.1.0"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2021"

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }
"#;

pub const BEVY_MAIN_RS_VALUE: &str = r#"fn main(){println!("hello");}"#;

pub const MINIMAL_MQ_CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
version = "0.1.0"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2021"

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }
anyhow = "1.0"
macroquad = "0.4.5"

[package.metadata.android]
app_wrapper = "quad"
"#;

pub const MQ_MAIN_RS_VALUE: &str = r#"
#[macroquad::main("Macroquad 3D")]
async fn main() -> anyhow::Result<()> {Ok(())}
"#;

pub const STRINGS_XML_VALUE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="hello">Hello!</string>
</resources>
"#;
