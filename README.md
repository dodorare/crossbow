<div>
<img src=".github/assets/splash.png" alt="Crossbow Splash Image" />

<a href="https://github.com/dodorare/crossbow/actions"><img alt="CI Info" src="https://github.com/dodorare/crossbow/workflows/CI/badge.svg"/></a>
<a href="https://crates.io/crates/crossbow"><img alt="Crate Info" src="https://img.shields.io/crates/v/crossbow.svg"/></a>
<a href="https://docs.rs/crossbow/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-crossbow-green"/></a>
<a href="https://crates.io/crates/crossbundle"><img alt="Tool Crate" src="https://img.shields.io/crates/d/crossbundle?label=cargo%20installs"/></a>
<a href="https://github.com/dodorare/crossbow/releases"><img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/dodorare/crossbow/total?label=binary%20downloads"/></a>
<a href="https://app.fossa.com/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow?ref=badge_shield" alt="FOSSA Status"><img src="https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow.svg?type=shield"/></a>

<strong>Cross-Platform Rust Toolkit for Games 🏹</strong>
</div>

## What is Crossbow?

A goal of the `crossbow` project is to provide a complete infrastructure for game development in rust. In addition, the project simplifies the creation and packaging of crates for Android, iOS, and other platforms. We want to make most of our tools - engine agnostic, to help rust game developers integrate them into their games, engines, and crates.

## Documentation

To learn how to run an example project on your own, build, test, and start using `crossbow` - read [our documentation](./docs/README.md).

## Project structure

Crate structure:

| Name | Description | Status |
| ---- | ----------- | ------ |
| [crossbundle](./crossbundle/cli/README.md) | Command-line tool for building applications | ✅ |
| [crossbundle-tools](./crossbundle/tools/README.md) | Toolkit used in `crossbundle` to build/pack/sign bundles | ✅ |
| [crossbundle-derive](./crossbundle/derive/README.md) | Derive macros for projects built with `crossbow` | ✅ |
| [crossbow-ads](./crossbow/ads/README.md) | Plugin for advertisements | 🛠 |
| [crossbow-permissions](./crossbow/permissions/README.md) | Plugin for runtime permissions | 🛠 |
| [android-tools-rs](https://github.com/dodorare/android-tools-rs) | Android-related tools for building and developing application | ✅ |
| [android-manifest-rs](https://github.com/dodorare/android-manifest-rs) | [AndroidManifest](https://developer.android.com/guide/topics/manifest/manifest-intro) serializer and deserializer for Rust | ✅ |
| [apple-bundle-rs](https://github.com/dodorare/apple-bundle-rs) | [AppleBundleResources](https://developer.apple.com/documentation/bundleresources) serializer and deserializer for Rust | ✅ |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development

## Roadmap

Also, check out our [ROADMAP](./ROADMAP.md) for a better understanding of what we are doing right now and what planned.

## Partners

This project is [part](https://github.com/w3f/Grants-Program/blob/master/applications/crossbow.md) of Web3 Foundation Grants Program.

<img src=".github/assets/w3f_grants_badge.svg" alt="W3F Grants Badge" width="400px" />

## License

Licensed under [Apache-2.0 License](LICENSE).

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fdodorare%2Fcrossbow?ref=badge_large)
