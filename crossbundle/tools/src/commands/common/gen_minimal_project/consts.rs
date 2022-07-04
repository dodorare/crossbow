pub const BEVY_CARGO_TOML_VALUE: &str = r#"
[package]
name = "bevy-2d"
version = "0.1.3"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2021"

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }
anyhow = "1.0"
bevy = { version = "0.7.0", features = ["mp3"] }

[package.metadata]
app_name = "Bevy 2D"
version_code = 1
target_sdk_version = 30
"#;

pub const BEVY_MAIN_RS_VALUE: &str = r#"fn main(){println!("hello");}"#;

pub const MQ_CARGO_TOML_VALUE: &str = r#"
[package]
name = "macroquad-permissions"
version = "0.1.3"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2021"

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow"}
anyhow = "1.0"
macroquad = "0.3.7"

[package.metadata]
app_name = "Macroquad_Permissions"
target_sdk_version = 30
version_code = 1
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
