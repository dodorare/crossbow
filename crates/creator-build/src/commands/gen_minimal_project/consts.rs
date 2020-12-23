pub const CARGO_TOML_VALUE: &'static str = r#"
[package]
name = "example"
edition = "2018"
version = "0.1.0"

[lib]
name = "examplelib"
crate-type = ["lib", "cdylib"]

[dependencies]
creator = { git = "https://github.com/creator-rs/creator" }
"#;

pub const LIB_RS_VALUE: &'static str = "#[creator::creator_main] pub fn main() {}";

pub const MAIN_RS_VALUE: &'static str = "fn main(){examplelib::main();}";
