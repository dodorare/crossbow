#[cfg(target_os = "android")]
use android_logger::Config;

use bevy::{prelude::*, render::pass::ClearColor};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full", logger(level = "trace")))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_min_level(log::Level::Trace));

    println!("The world!");
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_default_plugins()
        .run();
}
