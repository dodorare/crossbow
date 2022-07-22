# [![Crossbow Splash Image](.github/assets/splash.png)](https://github.com/dodorare/crossbow)

[![CI Info](https://github.com/dodorare/crossbow/workflows/CI/badge.svg)](https://github.com/dodorare/crossbow/actions)
[![Crate Info](https://img.shields.io/crates/v/crossbow.svg)](https://crates.io/crates/crossbow)
[![Documentation](https://img.shields.io/badge/docs.rs-crossbow-green)](https://docs.rs/crossbow/)
[![Crossbundle Crate](https://img.shields.io/crates/d/crossbundle?label=cargo%20installs)](https://crates.io/crates/crossbundle)
[![Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow?ref=badge_shield)
[![GitHub Stars](https://img.shields.io/github/stars/dodorare/crossbow.svg?style=social)](https://github.com/dodorare/crossbow/stargazers)

## What is Crossbow?

The `crossbow` project aims to provide a complete toolkit for cross-platform game development in Rust - from project creation to publishing. In addition, the project simplifies the creation, packaging, and signing of Android and iOS applications. We want to make most of our tools - engine agnostic to help rust game developers integrate them into their engines or games.

## Why Crossbow?

> There are already [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk), [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile), [cargo-xcode](https://gitlab.com/kornelski/cargo-xcode), etc. - why do I need another packaging tool?

Project `crossbow` is not only a packaging tool for Android and iOS - it's a toolkit. With `crossbundle-tools` you can customize and create new commands; with `crossbundle` you can create native **.apk/.aab** without any *Java* or setup *Gradle* project with fancy **Crossbow Android plugins** (**iOS** in near future); with `crossbow-android` you can write your own Android plugins in *Java/Kotlin*.

## Design Goals

* **Customizable**: Create new commands with available tools.
* **Simple**: Easy to start but flexible for strong devs.
* **Capable**: It's possible to build plain **.apk/.aab** or **.app/.ipa**; or with help of *Gradle/XCode*.
* **Rust**: Don't leave your *Rust* code - almost everything can be configured from **Cargo.toml**.

## 🛠 Installation

To install crossbundle, run:

```sh
cargo install --git=https://github.com/dodorare/crossbow crossbundle
```

See [installation documentation](./docs/README.md) for more details on how to setup environment on your platform.

## 🗂️ Project structure

Crossbundle crates:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [crossbundle](./crossbundle/cli) | Command-line tool for building and running applications. | ✅ |
| [crossbundle-tools](./crossbundle/tools) | Toolkit used in `crossbundle` to build/pack/sign bundles. | ✅ |

Crossbow Plugins:

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

## 📚 Documentation

To learn how to run an example project on your own, build, test, and start using `crossbow` - read our full documentation [here](./docs/README.md).

If you want to start development right away - see our Hello World example [here](./docs/main-hello-world.md).

If you want learn more about `crossbundle` we recommend that you read it's README [here](./crossbundle/cli/).

## 📅 Roadmap

Check out our [ROADMAP](./ROADMAP.md) for a better understanding of what we are doing right now and what planned.

## ✌️ Thanks and Alternatives

A lot of functionality was inspired by [godot](https://github.com/godotengine/godot), [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk), [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile).

Also, this project initially funded by [Web3 Foundation Grants Program](https://github.com/w3f/Grants-Program/blob/master/applications/crossbow.md). Big cheers to them!

<img src=".github/assets/w3f_grants_badge.svg" alt="W3F Grants Badge" width="400px" />

## 📑 License

Licensed under [Apache-2.0 License](LICENSE).

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow?ref=badge_large)
