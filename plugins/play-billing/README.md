# Crossbow Admob Plugin

[![Crate Info](https://img.shields.io/crates/v/play-billing.svg)](https://crates.io/crates/play-billing)
[![Documentation](https://img.shields.io/badge/docs.rs-play_billing-green)](https://docs.rs/play-billing/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dodorare/crossbow#license)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## About

This project is a Crossbow Plugin for [Google Play Billing](https://developer.android.com/google/play/billing) written in Rust and Kotlin.

## Installation

> **Important:** Before using this plugin please follow instructions on [Getting ready Google Play Billing](https://developer.android.com/google/play/billing/getting-ready) official guide.

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.2.2"
[target.'cfg(target_os = "android")'.dependencies]
play-billing = "0.2.2"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.play_billing:play_billing:0.2.2"]
```

> That's it, now you can start using Play Billing!

## Usage

First step is plugin initialization. In your rust project, you will need to initialize `Crossbow` instance and then get **Android** plugin:

```rust
#![cfg(target_os = "android")]

use crossbow::android::*;
let crossbow = CrossbowInstance::new();
let play_billing: play_billing::PlayBillingPlugin = crossbow.get_plugin()?;
```

After plugin initialization you can use supported features. For example to start connection and query purchases you can use:

```rust
play_billing.start_connection()?;
play_billing.query_purchases("YOUR_TYPE")?;
```

To read signals:

```rust
if let Ok(signal) = play_billing.get_receiver().recv().await {
    println!("Signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/play-billing/).

## Thanks and inspiration

This Plugin was initially inspired by [godot-google-play-billing](https://github.com/godotengine/godot-google-play-billing).
