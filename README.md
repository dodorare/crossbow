# [![Crossbow Splash Image](assets/splash.png)](https://github.com/dodorare/crossbow)

[![CI Info](https://github.com/dodorare/crossbow/workflows/CI/badge.svg)](https://github.com/dodorare/crossbow/actions)
[![Crate Info](https://img.shields.io/crates/v/crossbow.svg)](https://crates.io/crates/crossbow)
[![Documentation](https://img.shields.io/badge/docs.rs-crossbow-green)](https://docs.rs/crossbow/)
[![Crossbundle Crate](https://img.shields.io/crates/d/crossbundle?label=cargo%20installs)](https://crates.io/crates/crossbundle)
[![Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow?ref=badge_shield)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## What is Crossbow?

The `crossbow` project aims to provide a complete toolkit for cross-platform game development in *Rust* - from project creation to publishing. In addition, the project simplifies the creation, packaging, and signing of **Android** and **iOS** applications. We want to make most of our tools - engine agnostic to help rust game developers integrate them into their engines or games.

## Why Crossbow?

> There are already [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk), [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile), [cargo-xcode](https://gitlab.com/kornelski/cargo-xcode), etc. - why do I need another packaging tool?

Project `crossbow` is not only a packaging tool for **Android** and **iOS** - it's cross-platform build tools and toolkit for Rust! With `crossbundle` you can create native **.apk/.aab** without any *Java* or setup *Gradle* project with fancy **Crossbow Android plugins** (**iOS** in near future); with `crossbundle-tools` you can customize and create new commands; with `crossbow-android` you can write your own Android plugins in *Java/Kotlin*.

A lot of functionality was inspired by [Godot](https://github.com/godotengine/godot), [Xamarin](https://dotnet.microsoft.com/en-us/apps/xamarin), and [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk).

## Design Goals

* **Customizable**: Create new commands with available tools.
* **Simple**: Easy to install and start hacking but also pretty flexible for strong devs.
* **Capable**: It's possible to build plain **.apk/.aab** or **.app/.ipa**; or with help of *Gradle/XCode*.
* **Rust**: Don't leave your *Rust* code - **everything** can be configured from `Cargo.toml`.
* **Plugins**: Godot-like plugins for **Android** (and **iOS** in future) with *Rust* wrapper!

## 📚 Documentation

Now we would recommend you to read [crossbow's documentation](https://crossbow.dodorare.com/). Over there you will find how to set up development environment, install needed crates and how to use the tools.

## 🗂️ Project structure

Crossbundle crates:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [crossbundle](./crossbundle/cli) | Command-line tool for building and running applications. | ✅ |
| [crossbundle-tools](./crossbundle/tools) | Toolkit used in `crossbundle` to build/pack/sign bundles. | ✅ |

Crossbow crates:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [crossbow-android](./platform/android) | Crossbow Android Platform implementation. | 🆗 |
| [crossbow-ios](./platform/ios) | Crossbow iOS Platform implementation. | 🛠 |
| [crossbow-admob](./crossbow/admob) | Google AdMob Plugin for Android (iOS in future). | 🆗 |
| [crossbow-play-billing](./crossbow/play-billing) | Google Play Billing for Android. | 📝 |
| [crossbow-play-games-sdk](./crossbow/play-games-sdk) | Google Play Games Sdk for Android. | 📝 |

Helper crates:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [android-tools-rs](https://github.com/dodorare/android-tools-rs) | Android-related tools for building and developing application. | ✅ |
| [android-manifest-rs](https://github.com/dodorare/android-manifest-rs) | [AndroidManifest](https://developer.android.com/guide/topics/manifest/manifest-intro) serializer and deserializer for Rust. | ✅ |
| [apple-bundle-rs](https://github.com/dodorare/apple-bundle-rs) | [AppleBundleResources](https://developer.apple.com/documentation/bundleresources) serializer and deserializer for Rust. | ✅ |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development — 📝 = Planned

## Special Thanks

Also, this project initially funded by [Web3 Foundation Grants Program](https://github.com/w3f/Grants-Program/blob/master/applications/crossbow.md). Big shout-out to them!

<img src="assets/w3f_grants_badge.svg" alt="W3F Grants Badge" width="400px" />

## 📑 License

Licensed under [Apache-2.0 License](LICENSE).

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow?ref=badge_large)
