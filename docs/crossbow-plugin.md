# Crossbow plugins

To write new Crossbow Android plugin - we recommend to clone this repository and use `crossbow-admob` as template for your plugin.

To do so, run these commands:

```sh
git clone https://github.com/dodorare/crossbow
cp -r crossbow/plugins/admob ./my-awesome-plugin
cd ./my-awesome-plugin
```

In Rust project you will need to update **Cargo.toml** and code in *src/* folder.

Now in Android gradle project you will able to write your own Android plugin in Java or Kotlin!

## Building

To build Android Gradle plugin you need to run the following command:

```sh
gradle build
```

## Testing locally

To import your plugin into your game with Crossbow you can add following in **Cargo.toml**:

```toml
[[package.metadata.android.plugins_local_projects]]
include = ":my-awesome-plugin"
project_dir = "../my-awesome-plugin/android"
```

It will try to find your plugin by the specified path relative to your **Cargo.toml**.

## Publishing to Github Maven repository

To publish your plugin to Github Maven repository you need to get Personal Access Token from your Github account and update information in your `my-awesome-plugin/android/publish.gradle` file.

To read more about Github Maven repository visit [Publishing Github Maven packages](https://docs.github.com/en/actions/publishing-packages/publishing-java-packages-with-gradle).

As you setup everything - you can run the following command to publish your plugin to Maven repository:

```sh
USERNAME=<NAME> TOKEN=<TOKEN> gradle publish
```

To publish your plugin to [crates.io](https://crates.io/) - you can see this [official article](https://doc.rust-lang.org/cargo/reference/publishing.html).

## Using Published plugin

After successfully publishing your plugin you can use it in your game. To do so you will want to import it in **Cargo.toml**:

```toml
[dependencies]
my_awesome_plugin = "0.1.0"

[[package.metadata.android]]
plugins_remote = ["com.crossbow.awesome:my_awesome_plugin:0.1.0"]
```

That's it, now you can use your plugin in your game!
