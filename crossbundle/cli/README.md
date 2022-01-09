# CrossBundle CLI

![splash](https://github.com/dodorare/crossbow/blob/main/.github/assets/splash.png?raw=true)

The **crossbundle** is a command-line tool that encapsulates boring stuff of **Android** and **iOS** build/packaging processes and helps mobile developers to create and maintain applications written in **rust** programming language.

## Installation

```sh
cargo install --git=https://github.com/dodorare/crossbow crossbundle
```

---

**_NOTE_**

For the correct work of the tool, you need to set up a development environment (ex. install some libraries and tools - such as Android SDK, Android NDK, XCode, etc).
More information about how to set up the environment in the **Android setup** and **iOS setup** wiki pages.

---

## Syntax

To see the complete documentation for each command/subcommand you can write `-h` or `--help`:

```sh
crossbundle -h
crossbundle build android -h
crossbundle run android -h
```

### Command `crossbundle`

```sh
crossbundle <SUBCOMMAND> [FLAGS] [OPTIONS]
```

```text
USAGE:
    crossbundle [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --current-dir <CURRENT_DIR>    The current directory where to run all commands
    -h, --help                         Print help information
    -q, --quiet                        No output printed to stdout
    -v, --verbose                      A level of verbosity, and can be used multiple times
    -V, --version                      Print version information

SUBCOMMANDS:
    build    Starts the process of building/packaging/signing of the rust crate
    help     Print this message or the help of the given subcommand(s)
    log      Attach logger to device with running application
    new      Creates a new Cargo package in the given directory. Project will be ready to build
             with `crossbundle`
    run      Executes `build` command and then deploy and launches the application on the
             device/emulator
```

### Command `crossbundle build android`

```sh
crossbundle build android [FLAGS] [OPTIONS]
```

```text
USAGE:
    crossbundle build android [OPTIONS]

OPTIONS:
        --aab
            Generating aab. By default crossbow generating apk

        --all-features
            Activate all available features of selected package

        --example <EXAMPLE>
            Build the specified example

        --features <FEATURES>...
            Space or comma separated list of features to activate. These features only apply to the
            current directory's package. Features of direct dependencies may be enabled with `<dep-
            name>/<feature-name>` syntax. This flag may be specified multiple times, which enables
            all specified features

    -h, --help
            Print help information

        --no-default-features
            Do not activate the `default` feature of the current directory's package

        --quad
            Specifies to build macroquad game engine

        --release
            Build optimized artifact with the `release` profile

        --sign-key-alias <SIGN_KEY_ALIAS>
            Signing key alias

        --sign-key-pass <SIGN_KEY_PASS>
            Signing key password

        --sign-key-path <SIGN_KEY_PATH>
            Path to the signing key

        --target <TARGET>...
            Build for the given android architecture. Supported targets are: `armv7-linux-
            androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android`
            [default: aarch64-linux-android]

        --target-dir <TARGET_DIR>
            Directory for generated artifact and intermediate files
```

### Command `crossbundle build apple`

```sh
crossbundle build apple [FLAGS] [OPTIONS]
```

```text
USAGE:
    crossbundle build apple [OPTIONS]

OPTIONS:
        --all-features
            Activate all available features of selected package

        --bin <BIN>
            Specify custom cargo binary

        --example <EXAMPLE>
            Build the specified example

        --features <FEATURES>...
            Space or comma separated list of features to activate. These features only apply to the
            current directory's package. Features of direct dependencies may be enabled with `<dep-
            name>/<feature-name>` syntax. This flag may be specified multiple times, which enables
            all specified features

    -h, --help
            Print help information

        --identity <IDENTITY>
            The id of the identity used for signing. It won't start the signing process until you
            provide this flag

        --no-default-features
            Do not activate the `default` feature of the current directory's package

        --profile-name <PROFILE_NAME>
            Provisioning profile name to find in this directory:
            `~/Library/MobileDevice/Provisioning\ Profiles/`

        --profile-path <PROFILE_PATH>
            Absolute path to provisioning profile

        --quad
            Specifies to build macroquad game engine

        --release
            Build optimized artifact with the `release` profile

        --target <TARGET>...
            Build for the given apple architecture. Supported targets are: 'aarch64-apple-ios`,
            `armv7-apple-ios`, `armv7s-apple-ios`, `i386-apple-ios`, `x86_64-apple-ios` [default:
            aarch64-apple-ios]

        --target-dir <TARGET_DIR>
            Directory for generated artifact and intermediate files

        --team-identifier <TEAM_IDENTIFIER>
            The team identifier of your signing identity
```
