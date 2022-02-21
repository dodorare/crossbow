# Using subxt with bevy engine

1. You need to install crossbundle if you haven't already. See [documention](https://github.com/dodorare/crossbow/tree/main/docs) to install it and configure your project.

2. Specify substrate-subxt and bevy in your Cargo.toml. We prefer to use versions below:

```sh
[dependencies]
substrate-subxt = "0.15"
bevy = "0.6.0"
```

## Bevy explorer example

To learn how to use subxt with bevy engine, you can go to the examples/bevy-explorer.

## Installing on the device

You can deploy the application on your device with commands below:

To build APK and run it on the device using the command. If you want to build an application replaces `run` with `build`.

```sh
crossbundle run android
# or
crossbundle run apple
```

To build AAB and run it on the device using the command. If you want to build an application replaces `run` with `build`.

```sh
crossbundle run android --aab
# or
crossbundle run apple --aab
```

## Known issues

You can face the problem with jsonrpsee-utils library used in subxt. One of the solutions is to downgrade the version. Use the command below:

```sh
cargo update -p jsonrpsee-utils --precise 0.2.0-alpha.3
```
