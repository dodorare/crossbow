# Google Play Billing tutorial

> **Important:** First of all you need to install crossbundle if you haven't already. See [documention](/docs/src/install/README.md) to install it.

## Generate a project

`crossbundle` uses [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) to generate a new project. This means that you need to install it before we proceed.

```sh
cargo install cargo-generate
```

Then you can create a new project:

```sh
crossbundle new project-name --template quad
```

All supported templates you can watch [`here`](https://github.com/dodorare/crossbundle-templates) (each branch = template).

## Installation

> **Important:** Before using this plugin please follow instructions on [Getting ready Google Play Billing](https://developer.android.com/google/play/billing/getting-ready) official guide.

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.2.2"
[target.'cfg(target_os = "android")'.dependencies]
play-billing = "0.2.2"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.play_billing:play_billing:0.2.2"]
```

> That's it, now you can start using Play Billing!

## Usage

First step is plugin initialization. In your rust project, you will need to initialize `Crossbow` instance and then get **Android** plugin:

```rust
#![cfg(target_os = "android")]

use crossbow::android::*;
let crossbow = CrossbowInstance::new();
let play_billing: play_billing::PlayBillingPlugin = crossbow.get_plugin()?;
```

After plugin initialization you can use supported features. For example to start connection and query purchases you can use:

```rust
play_billing.start_connection()?;
play_billing.query_purchases("YOUR_TYPE")?;
```

To read signals:

```rust
if let Ok(signal) = play_billing.get_receiver().recv().await {
    println!("Signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/play-billing/).

## Build an application

> **Important:** Read officialtutorial on how to test Google Play Billing for more information [here](https://developer.android.com/google/play/billing/test).

Let's build and run this application. Android command below will generate gradle project and install apk on your device. See [crossbundle run command](/docs/src/crossbundle/command-run.md) for additional information.

```sh
crossbundle run android
```

But to be able to test In-app purchases - you will need to add Products in Google Play Console. And then build and sign your app with your generated keystore.

To publish your app to Play Store Internal Testing - you will want to export gradle project with this command:

```sh
crossbundle build android --export-path=./path/
```

Then in generated project add signing to `build.gradle` like this:

```
signingConfigs {
    release {
        if (project.hasProperty('MYAPP_UPLOAD_STORE_FILE')) {
            storeFile file(MYAPP_UPLOAD_STORE_FILE)
            storePassword MYAPP_UPLOAD_STORE_PASSWORD
            keyAlias MYAPP_UPLOAD_KEY_ALIAS
            keyPassword MYAPP_UPLOAD_KEY_PASSWORD
        }
    }
}
buildTypes {
    release {
        signingConfig signingConfigs.release
    }
}
```

and in `gradle.properties` actual values:

```
MYAPP_UPLOAD_STORE_FILE=my-project.keystore
MYAPP_UPLOAD_KEY_ALIAS=my-project
MYAPP_UPLOAD_STORE_PASSWORD=123456
MYAPP_UPLOAD_KEY_PASSWORD=123456
```

You can read more [here](https://reactnative.dev/docs/signed-apk-android), and [here](https://developer.android.com/studio/publish).
