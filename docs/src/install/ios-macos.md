# iOS setup on macOS

Crossbundle requires the full Xcode installation; the standalone Command Line Tools are not
enough. Install Xcode from the [Mac App Store](https://apps.apple.com/app/xcode/id497799835),
launch it once to finish setup, and install an iOS Simulator runtime through Xcode.

If another developer directory is active, select Xcode explicitly:

```sh
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

Install Rust, Crossbundle, and the supported iOS targets:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
cargo install --git https://github.com/dodorare/crossbow crossbundle
```

Intel Macs also need the x86_64 Simulator target:

```sh
rustup target add x86_64-apple-ios
```

Verify the toolchain before building:

```sh
crossbundle doctor --platform apple
```

## Build and run in Simulator

From a Crossbundle project, build or run with:

```sh
crossbundle build ios
crossbundle run ios
```

Crossbundle builds for the host's Simulator architecture and selects an available iOS
Simulator automatically. Pass `--simulator <NAME_OR_UDID>` to select one explicitly, or
`--no-open --detach` for automation.

## Run on a physical device

Physical-device deployment requires:

- an Apple signing certificate installed in the login keychain;
- a provisioning profile matching the application's bundle identifier;
- the Apple Developer Team ID associated with both;
- [`ios-deploy`](https://github.com/ios-control/ios-deploy), installed with
  `brew install ios-deploy`.

List available signing identities with:

```sh
security find-identity -v -p codesigning
```

Then pass the profile by absolute path, the Team ID, and the certificate name or SHA-1 hash:

```sh
crossbundle run ios --release --device \
  --profile-path=/absolute/path/to/profile.mobileprovision \
  --team-id=AS9UV719T7 \
  --signing-identity=AF96DABFC5DEE81E339ED8755DA8D1E48A87CBFE
```
