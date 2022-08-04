# Using subxt with bevy engine

1. You need to install crossbundle if you haven't already. See [documention](https://github.com/dodorare/crossbow/tree/main/docs) to install it and configure your project.

2. Specify substrate-subxt and bevy in your Cargo.toml. We prefer to use versions below:

```sh
[dependencies]
substrate-subxt = "0.21"
bevy = "0.7.0"
```

## Bevy explorer example

To learn how to use subxt with bevy engine, you can go to the [examples/bevy-explorer](https://github.com/dodorare/crossbow/tree/main/examples/bevy-explorer) or install bevy explorer template. Follow next steps:

1. Install cargo-generate:

```sh
cargo install cargo-generate
```

2. Install bevy-explorer template:

```sh
crossbundle new example --template=bevy-explorer
```

3. After previous steps, now you can install the application on the device.

## Installing application on the device

You can deploy the application on your device with commands below. At first, you should go to example directory. Use it:

Bash:

```sh
# If the template was installed
cd example
# If bevy-explorer example will be used
cd example/bevy-explorer
```
By default `crossbundle build android` command will generate gradle project and install apk on your device. To build native `.apk` and `.aab` see commands below.
To build native APK and run it on the device using the command. If you want to build an application replaces `run` with `build`.

```sh
crossbundle run android -s=native-apk
# or
crossbundle run ios
```

To build native AAB and run it on the device using the command. If you want to build an application replaces `run` with `build`.

```sh
crossbundle run android -s=native-aab
```