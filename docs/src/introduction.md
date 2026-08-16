# Introduction

![splash](https://github.com/dodorare/crossbow/blob/main/assets/crossbow/splash.png?raw=true)

## What is Crossbow?

The `crossbow` project aims to provide a complete toolkit for cross-platform game development in *Rust* - from project creation to publishing. In addition, the project simplifies the creation, packaging, and signing of **Android** and **iOS** applications. We want to make most of our tools - engine agnostic to help rust game developers integrate them into their engines or games.

## Why Crossbow?

> There are already [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk), [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile), [cargo-xcode](https://gitlab.com/kornelski/cargo-xcode), etc. - why do I need another packaging tool?

Crossbow is more than an **Android** and **iOS** packager: it is a cross-platform Rust game-development toolkit. `crossbundle` turns standard Cargo projects into native **.apk/.aab** and **.app/.ipa** artifacts, with optional Gradle packaging for Android plugins. `crossbundle-tools` provides the same building blocks for custom workflows, while `crossbow-android` supports Android plugins written in *Java/Kotlin*.

A lot of functionality was inspired by [Godot](https://github.com/godotengine/godot), [Xamarin](https://dotnet.microsoft.com/en-us/apps/xamarin), and [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk).

## Design Goals

* **Customizable**: Create new commands with available tools.
* **Simple**: Easy to install and start hacking but also pretty flexible for strong devs.
* **Cargo-first**: Builds use Cargo's public CLI and consume the artifacts Cargo reports.
* **Capable**: Build native **.apk/.aab** and **.app/.ipa** artifacts, with optional Gradle packaging on Android.
* **Rust**: Don't leave your *Rust* code - **everything** can be configured from `Cargo.toml`.
* **Plugins**: Godot-like plugins for **Android** (and **iOS** in future) with *Rust* wrapper!

## Next steps

As the next steps we recommend you to install and setup `crossbundle` to be able to build, test, and run your project!

See [Getting Started](install/README.md) for more information.
