pub const CARGO_TOML_VALUE: &'static str = r#"
[package]
name = "example"
edition = "2018"
version = "0.1.0"

[lib]
name = "examplelib"

[dependencies]
# bevy = { git = "https://github.com/bevyengine/bevy", rev = "61b181a699ed2b450bebc8c14d96c6af55fa41cf" }
# bevy = "0.3"
# creator = { version = "0.2.1", path = "../../" }
"#;

pub const LIB_RS_VALUE: &'static str = r#"
// use bevy::prelude::*;

// #[bevy::bevy_main]
pub fn main() {
//    App::build().run();
}
"#;

pub const MAIN_RS_VALUE: &'static str = r#"
fn main() {
    examplelib::main();
}
"#;
