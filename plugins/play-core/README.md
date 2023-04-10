# Crossbow Google Play Core Plugin

[![Crate Info](https://img.shields.io/crates/v/play-core.svg)](https://crates.io/crates/play-core)
[![Documentation](https://img.shields.io/badge/docs.rs-play_core-green)](https://docs.rs/play-core/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dodorare/crossbow#license)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## About

This project is a Crossbow Plugin for [Google Play Core libraries](https://developer.android.com/guide/playcore) written in Rust and Kotlin.

## Installation

> **Important:** Before using this plugin please read more about [Google Play Core libraries](https://developer.android.com/guide/playcore) and [In-app updates](https://developer.android.com/guide/playcore/in-app-updates).

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.2.4"
[target.'cfg(target_os = "android")'.dependencies]
play-core = "0.2.4"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.play_core:play_core:0.2.3"]
```

> That's it, now you can start using Play Core!

## Usage

First step is plugin initialization. In your rust project, you will need to initialize `Crossbow` instance and then get **Android** plugin:

```rust
#![cfg(target_os = "android")]

use crossbow::android::*;
let crossbow = CrossbowInstance::new();
let play_core: play_core::PlayCorePlugin = crossbow.get_plugin()?;
```

After plugin initialization you can use supported features. For example to start connection and query purchases you can use:

```rust
play_core.check_update()?;
play_core.in_progress_update()?;
```

To read signals:

```rust
if let Ok(signal) = play_core.get_receiver().recv().await {
    println!("Signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/play-core/).

## Future work

Ideally we will get rid of the Java wrapper and will create C++ wrapper around [Google Play Core native](https://developer.android.com/reference/native/play/core) - so that it will support all features and will work faster than with JNI. If you want to help us with it - welcome!
