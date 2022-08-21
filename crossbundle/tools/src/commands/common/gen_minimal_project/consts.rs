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
macroquad = "0.3"

[package.metadata.android]
app_wrapper = "sokol"
"#;

pub const MINIMAL_MQ_GRADLE_CARGO_TOML_VALUE: &str = r#"
[package]
name = "example"
version = "0.1.0"
authors = ["DodoRare Team <support@dodorare.com>"]
edition = "2021"

[dependencies]
crossbow = { git = "https://github.com/dodorare/crossbow" }
anyhow = "1.0"
macroquad = "0.3"

[package.metadata.android]
app_wrapper = "sokol"
target_sdk_version = 30

[[package.metadata.android.plugins_local_projects]]
include = ":crossbow"
dont_implement = true
project_dir = "../../platform/android/java"

[[package.metadata.android.plugins_local_projects]]
include = ":crossbow:lib"
"#;

pub const CARGO_TOML_VALUE: &str = r#"
[package.metadata.android]
app_name = "Example"
release_build_targets = ["aarch64-linux-android"]

[package.metadata.android.manifest]
package = "com.crossbow.example"
[package.metadata.android.manifest.uses_sdk]
min_sdk_version = 19
target_sdk_version = 30

[[package.metadata.android.manifest.uses_feature]]
name = "android.hardware.vulkan.level"
required = true
version = 1

[[package.metadata.android.manifest.permission]]
name = "android.permission.WRITE_EXTERNAL_STORAGE"

[[package.metadata.android.manifest.uses_permission_sdk_23]]
name = "android.permission.INTERNET"

[package.metadata.apple]
app_name = "Macroquad_3D"
release_build_targets = ["aarch64-apple-ios", "x86_64-apple-ios"]
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
