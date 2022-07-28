# Crossbow Admob Plugin

## About

This project is a Crossbow Plugin that allows showing AdMob ads from Rust. Without worrying about the building, just download and use.

## Features

| Ad Format | Available |
| ---- | ----------- |
| Banner | ❌ |
| Interstitial | ✅ |
| Rewarded | ✅ |
| [Rewarded Interstitial](https://support.google.com/admob/answer/9884467) | ✅ |
| Native | ❗ |

✅ = Works and tested — 🆗 = Works but may contain bugs — 🛠 = Under development — 📝 = Planned - ❌ = Not working - ❗ = Not planned to be implemented

## Installation

Just add Rust dependencies like this:

```toml
[dependencies]
crossbow = "0.1.7"
crossbow-admob = "0.1.7"
```

And finally, add this to your Crossbow Android configuration:

```toml
[package.metadata.android]
plugins_remote = ["com.crossbow.admob:admob:0.1.7"]
```

> That's it, now you can start using AdMob ads!

If you want to configure custom APPLICATION_ID add this to your Cargo.toml file:

```toml
[[package.metadata.android.meta_data]]
name = "com.google.android.gms.ads.APPLICATION_ID"
value = "<YOUR ID HERE>"
# By default: ca-app-pub-3940256099942544~3347511713
```

## Usage

In your rust project, you will need to get JNIEnv first and retrieve the JNI Singleton instance of AdMob from Crossbow. To do this, write following code:

```rust
use crossbow::android::{permission::*, plugin};

let (_, vm) = crossbow::android::create_java_vm().unwrap();
let jnienv = vm.attach_current_thread_as_daemon().unwrap();

let admob_singleton = plugin::get_jni_singleton("AdMob").expect("Crossbow Error: AdMob is not registered");
let admob = crossbow_admob::AdMobPlugin::from_jnienv(admob_singleton.clone(), jnienv).unwrap();
```

To show Interstitial Ad, use following code:

```rust
admob.initialize(true, "G", false, true).unwrap();
admob.load_interstitial("ca-app-pub-3940256099942544/1033173712").unwrap();
admob.show_interstitial().unwrap();
```

To read signals:

```rust
if let Ok(signal) = admob_singleton.get_receiver().recv().await {
    println!("signal: {:?}", signal);
}
```

Complete documentation you can find [here](https://docs.rs/crossbow-admob/).

## Thanks and inspiration

This Plugin was initially inspired by [godot-admob-android](https://github.com/Poing-Studios/godot-admob-android).
