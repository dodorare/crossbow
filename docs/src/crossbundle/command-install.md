# Setup packages with crossbundle install

Use `crossbundle` install command to install necessary packages. To find out available commands specify the -h flag.

```sh
crossbundle install -h
```

## Install tools to APK correct building

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

To install packages use the command below. We prefer to use --preferred-tools flag to install minimal required tools needed for crossbundle correct working. This command will setup build-tools, android-ndk and android platforms: 

```sh
crossbundle install sdk-manager --preferred-tools
```

Also you can install packages manually. To see all available tools use the -h flag. List installed and available packages:

```sh
crossbundle install sdk-manager --list
```

And then enter the command.

```sh
crossbundle install sdk-manager --install "build-tools;31.0.0" "ndk;23.1.7779620" "platforms;android-31"
```

The command will install packages into `$HOME\AppData\Local\Android\Sdk\` for Windows, `$HOME/Library/Android/sdk/` for macOS, and `$HOME/Android/sdk/` for Linux.

## Install tools to AAB correct building

For correct AAB building install [bundletool](https://developer.android.com/studio/command-line/bundletool) and tools above.

### Install bundletool

To install [bundletool](https://developer.android.com/studio/command-line/bundletool) use command below. To see all available options use the -h flag.

```sh
crossbundle install bundletool
```

The command will download bundletool from [`GitHub repository`](https://github.com/google/bundletool/releases) and save it into `$HOME`. Notice, that you should install [Java JDK](https://www.oracle.com/java/technologies/downloads/) to open bundletool jar file.
