# Crossbundle install command 

## Setup packages 

Use `crossbundle` install command to install necessary packages. To find out available commands specify the `-h` flag. The `-h` flag can be used in all subcommands crossbundle install offers.

```sh
crossbundle install -h
crossbundle install command-line-tools -h
```

## Install tools to APK building

We offer to use our command to fast installation all required packages.

```sh
crossbundle install --preferred
```

This command will setup command line tools, Android platforms, build-tools, Android NDK and bundletool for AAB correct working. To provide custom installation read the article below. 

### Install command-line tools

If you do not need Android Studio, you can download the basic Android [command line tools](https://developer.android.com/studio/command-line) below. You can use the included [sdkmanager](https://developer.android.com/studio/command-line/sdkmanager) to download other SDK packages.

These tools are included in Android Studio.

To see all available options use the -h flag. To install command line tools use the command:

```sh
crossbundle install command-line-tools
```

The command will download a zip archive and unzip command line tools into `$HOME\AppData\Local\Android\Sdk\cmdline-tools\bin` for windows and `$HOME/Local/Android/Sdk/cmdline-tools/bin` for other operating systems.
Note: Android studio install cmdline tools into `$SDK_ROOT/cmdline-tools/<version>/bin`.

### Install packages

The [sdkmanager](https://developer.android.com/studio/command-line/sdkmanager) is a command-line tool that allows you to view, install, update, and uninstall packages for the Android SDK.

Also you can install packages manually. To see all available tools use the -h flag. List installed and available packages:

```sh
crossbundle install sdkmanager --list
```

And then enter the command.

```sh
crossbundle install sdkmanager --install "build-tools;31.0.0" "ndk;23.1.7779620" "platforms;android-31"
```

The command will install packages into `$HOME\AppData\Local\Android\Sdk\` for Windows, `$HOME/Library/Android/sdk/` for macOS, and `$HOME/Android/sdk/` for Linux.

## Install bundletool to AAB building

To install [bundletool](https://developer.android.com/studio/command-line/bundletool) use command below. To see all available options use the -h flag.

```sh
crossbundle install bundletool
```

The command will download bundletool from [`GitHub repository`](https://github.com/google/bundletool/releases) and save it into `$HOME`. Notice, that you should install [Java JDK](https://www.oracle.com/java/technologies/downloads/) to open bundletool jar file.
