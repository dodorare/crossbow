# Crossbow Play Games Services Plugin

[![Crate Info](https://img.shields.io/crates/v/play-games-services.svg)](https://crates.io/crates/play-games-services)
[![Documentation](https://img.shields.io/badge/docs.rs-play-games-services-green)](https://docs.rs/play-games-services/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/dodorare/crossbow#license)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## About

This project is a Crossbow Plugin for [Google Play Games Services](https://developers.google.com/games/services) written in Rust and Kotlin.

The Android implementation uses the Play Games Services v2 SDK and its automatic
authentication flow.
See Google's [v1 deprecation and v2 migration notice](https://developers.google.com/android/guides/releases#june_11_2026).

### Identity migration

The `_on_sign_in_success` signal now contains a **Play Games Player ID**, not the
Google account ID returned by Crossbow's old v1 integration. Treat this value as a
secondary platform identifier for achievements, leaderboards, events, and saved games.
Do not use it as the primary key for player progress, inventory, or currency.

Before releasing this migration for a game that associated the old signal value with an
in-game account, add an explicit account-linking migration. Google recommends a stable
OpenID from Sign in with Google or an independent account system as the primary identity,
with the Play Games Player ID retained only as a secondary association. See the
[PGS identity migration overview](https://developer.android.com/games/pgs/migration_overview).

### Supported features:

| Feature | Available |
| ---- | ----------- |
| Sign-in | ✅ |
| Programmatic sign-out | ❌ (not available in PGS v2) |
| Achievements | 🆗 |
| Leaderboards | 🆗 |
| Events | 🆗 |
| Player Stats | 🆗 |
| Player Info | 🆗 |
| Saved Games | 🆗 |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development — 📝 = Planned - ❌ = Not working - ❗ = Not planned to be implemented

## Installation

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.2.3"
[target.'cfg(target_os = "android")'.dependencies]
play-games-services = "0.2.3"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.play_games_services:play_games_services:0.2.3"]
```

## Getting started

> **Important:** Before using this plugin please follow instructions on [Setting Up Google Play Games Services](https://developers.google.com/games/services/console/enabling) official guide.

### Setting up `AndroidManifest` resources

Create you resources directory and import it by adding the following in your `Cargo.toml`:

```toml
[package.metadata.android]
resources = ["./res/android"]
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

After plugin initialization you can request sign-in explicitly. PGS v2 also attempts
automatic authentication during initialization:

```rust
play_games.sign_in()?;
```

Successful authentication emits `_on_sign_in_success` with the Play Games Player ID.
Programmatic sign-out is unavailable in PGS v2; users manage the persistent platform
profile through Android and Play Games settings.

To read signals:

```rust
if let Ok(signal) = play_games.get_receiver().recv().await {
    println!("Signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/play-games-services/).

## Troubleshooting

1. If you use **Android Emulator** - make sure that you use one that supports [Google Play Games Services](https://developers.google.com/games/services). See similar [StackOverflow question](https://stackoverflow.com/questions/34653347/using-google-play-games-services-in-emulator).
2. If authentication fails, verify the package name, SHA fingerprint, Play Games app ID,
   and linked Play Console configuration.
3. Make sure that you sign your application with the certificate registered in Play Console.

## Thanks and inspiration

This Plugin was initially adapted and inspired by [godot-pgsgp](https://github.com/cgisca/PGSGP).
