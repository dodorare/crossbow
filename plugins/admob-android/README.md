# Crossbow Admob Plugin

[![Crate Info](https://img.shields.io/crates/v/admob-android.svg)](https://crates.io/crates/admob-android)
[![Documentation](https://img.shields.io/badge/docs.rs-admob_android-green)](https://docs.rs/admob-android/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dodorare/crossbow#license)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## About

This project is a Crossbow Plugin that allows showing AdMob ads from Rust. Without worrying about the building, just download and use.

### Supported features

| Ad Format | Available |
| ---- | ----------- |
| Banner | ❌ (probably doesn't work with **NativeActivity**) |
| Interstitial | ✅ |
| Rewarded | ✅ |
| [Rewarded Interstitial](https://support.google.com/admob/answer/9884467) | ✅ |
| Native | ❗ |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development — 📝 = Planned - ❌ = Not working - ❗ = Not planned to be implemented

## Installation

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.2.4"
[target.'cfg(target_os = "android")'.dependencies]
admob-android = "0.2.4"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.admob:admob:0.2.3"]
```

> That's it, now you can start using AdMob ads!

If you want to publish or share your application to show real ads - configure custom APPLICATION_ID through `Cargo.toml` file:

```toml
[[package.metadata.android.manifest.application.meta_data]]
name = "com.google.android.gms.ads.APPLICATION_ID"
value = "<YOUR ID HERE>"
# By default: ca-app-pub-3940256099942544~3347511713
```

## Usage

First step is plugin initialization. In your rust project, you will need to initialize `Crossbow` instance and then get **Android** plugin:

```rust
#![cfg(target_os = "android")]

use crossbow::android::*;
let crossbow = CrossbowInstance::new();
let admob: admob_android::AdMobPlugin = crossbow.get_plugin()?;
// Initialize AdMob Service
admob.initialize(true, "G", false, true).unwrap();
```

To show Interstitial Ad, use following code (remember, currently there's no async API for this plugin - so `load` and `show` functions should be called as soon as `Sinals` received or `is_initialized()/is_interstitial_loaded()` checked):

```rust
admob.load_interstitial("ca-app-pub-3940256099942544/1033173712").unwrap();
admob.show_interstitial().unwrap();
```

The result will be like this:

![AdMob Ad Result Example](../../assets/images/admob-example.png)

To read signals:

```rust
if let Ok(signal) = admob.get_receiver().recv().await {
    println!("Signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/admob-android/).

## Thanks and inspiration

This Plugin was initially inspired by [godot-admob-android](https://github.com/Poing-Studios/godot-admob-android).
