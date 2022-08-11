# Introduction

![splash](https://github.com/dodorare/crossbow/blob/main/assets/crossbow/splash.png?raw=true)

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

## Next steps

As the next steps we recommend you to install and setup `crossbundle` to be able to build, test, and run your project!

See [Getting Started](install/README.md) for more information.
