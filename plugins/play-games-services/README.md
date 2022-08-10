# Crossbow Admob Plugin

[![Crate Info](https://img.shields.io/crates/v/play-games-services.svg)](https://crates.io/crates/play-games-services)
[![Documentation](https://img.shields.io/badge/docs.rs-play-games-services-green)](https://docs.rs/play-games-services/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dodorare/crossbow#license)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## About

This project is a Crossbow Plugin for [Google Play Games Services](https://developers.google.com/games/services) written in Rust and Kotlin.

### Supported features:

| Feature | Available |
| ---- | ----------- |
| Sign-in/Sign out | ✅ |
| Achievements | 🆗 |
| Leaderboards | 🆗 |
| Events | 🆗 |
| Player Stats | 🆗 |
| Player Info | 🆗 |
| Saved Games | 🆗 |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development — 📝 = Planned - ❌ = Not working - ❗ = Not planned to be implemented

## Getting started

> **Important:** Before using this plugin please follow instructions on [Setting Up Google Play Games Services](https://developers.google.com/games/services/console/enabling) official guide.

### Setting up `AndroidManifest` resources

Create you resources directory and import it by adding the following in your `Cargo.toml`:

```toml
[package.metadata.android]
res = "./res/android"
```

Then create file `./res/android/values/games-ids.xml` in your resources directory with the following content:

```xml
<?xml version="1.0" encoding="utf-8"?>
<!--Google Play game services IDs. Save this file as res/values/games-ids.xml in your project.-->
<resources>
    <!--app_id-->
    <string name="app_id" translatable="false">ADD_YOUR_APP_ID</string>
</resources>
```

> **Important:** You need to replace `ADD_YOUR_APP_ID` with your app ID. Read [official instructions](https://developers.google.com/games/services/console/enabling) to learn more.

Next, specify the package and meta_data for the Application's AndroidManifest.xml in your `Cargo.toml`:

```toml
[package.metadata.android.manifest]
package = "com.crossbow.play_games"
[[package.metadata.android.manifest.application.meta_data]]
name = "com.google.android.gms.games.APP_ID"
value = "@string/app_id"
[[package.metadata.android.manifest.application.meta_data]]
name = "com.google.android.gms.version"
value = "@integer/google_play_services_version"
```

## Installation

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.2.0"
[target.'cfg(target_os = "android")'.dependencies]
play-games-services = "0.2.0"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.play_games_services:play_games_services:0.2.0"]
```

> That's it, now you can start using Play Games Services!

## Usage

First step is plugin initialization. In your rust project, you will need to initialize `Crossbow` instance and then get **Android** plugin:

```rust
#![cfg(target_os = "android")]

use crossbow::android::*;
let crossbow = CrossbowInstance::new();
let play_games: play_games_services::PlayGamesServicesPlugin = crossbow.get_plugin()?;
// Initialize Google Play Games Services
play_games.init(true)?;
```

After plugin initialization you can use supported features. For example to SignIn user you can use:

```rust
play_games.sign_in()?;
```

To read signals:

```rust
if let Ok(signal) = play_games.get_receiver().recv().await {
    println!("Signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/play-games-services/).

## Troubleshooting

1. If you use **Android Emulator** - make sure that you use one that supports [Google Play Games Services](https://developers.google.com/games/services). See similar [StackOverflow question](https://stackoverflow.com/questions/34653347/using-google-play-games-services-in-emulator).
2. If you keep getting `Error 12501` - make sure that fingerprint, package, and resources are configured correctly. See similar [StackOverflow question](https://stackoverflow.com/questions/62973082/android-google-play-games-signin-error-12501).
3. If you keep getting `Error 4` - make sure that you sign your Application with correct Play Store key.

## Thanks and inspiration

This Plugin was initially adapted and inspired by [godot-pgsgp](https://github.com/cgisca/PGSGP).
