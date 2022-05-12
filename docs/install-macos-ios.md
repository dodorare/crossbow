# 📱 iOS setup on MacOS

## ⚙️ Setup on macOS

Install `brew`:

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Install `rust` with `default` option:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Source binaries
echo 'source $HOME/.cargo/env' >> ~/.zshrc
source ~/.zshrc
# Or for bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

Install iOS targets:

```sh
# 64-bit targets (real device & simulator):
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
# 32-bit targets (you probably don't need these & nightly only):
rustup target add armv7-apple-ios armv7s-apple-ios i386-apple-ios
```

Install `ios-deploy` for installing your app on iPhone:

```sh
brew install ios-deploy
```

Install `Xcode` from [App Store](https://apps.apple.com/us/app/xcode/id497799835).

<img alt="Xcode installation" src="https://i.imgur.com/2RhOz1t.png" width="400px"></img>

Install the necessary `Xcode tools` using `Xcode`:

1. Start `Xcode`.
2. Choose `Preferences` from the `Xcode` menu.
3. In the `Locations` panel, find the `Command Line Tools` field.
4. Click on the `Command Line Tools` and select `Xcode` version (you are asked for your Apple Developer login during the install process).

Login with Free Apple Developer/Apple Developer Program account:

1. Start `Xcode`.
2. Choose `Preferences` from the `Xcode` menu.
3. In the `Accounts` panel, click the `+` sign (you are asked for your Apple Developer login during the install process).
4. Click on Apple ID added account and find your name with `(Personal team)` postfix.
5. Open `Manage Certificates` then click `+` and `Apple Development`.
6. Close by clicking `Done` and click on `Download Manual Profiles`.
iOS setup on MacOS

<img alt="Xcode Sign in" src="https://i.imgur.com/soD5gab.png" width="400px"></img>

Install `crossbundle`:

```sh
cargo install --git=https://github.com/dodorare/crossbow crossbundle
```

## 🔨 Build Apple app

Now everything is ready to build your application.

To simplify this example we will create a new cargo project by this command:

```sh
crossbundle new game --template bevy
cd game/
```

Now run the build command:

```sh
crossbundle build apple --target=x86_64-apple-ios
```

And if you want to run on the simulator:

```sh
crossbundle run apple --target=x86_64-apple-ios
```

## 🏃‍♀️ Run your app on a real device

To run your application on a real device you will need to open `Xcode` and create a new project
with the same `bundle_identifier` as your app. Then you will find a `mobileprovision` file
in the `~/Library/MobileDevice/Provisioning\ Profiles` folder (e.x: aec73e2f-c2f9-4e3b-9393-be19cc52fea3.mobileprovision).

With command `security find-identity -p codesigning -v` - you will get a similar result:

```sh
$ security find-identity -p codesigning -v
  1) AF96DABFC5DEE81E339ED8755DA8D1E48A87CBFE "Apple Development: <your-email>@gmail.com (TRGW43YM8W)"
     1 valid identities found
```

The `AF96DABFC5DEE81E339ED8755DA8D1E48A87CBFE` value is our sign Identity. Save it for later use.

Copy your code (in our case `TRGW43YM8W`) and place it in another command to get Team ID:

```sh
$ security find-certificate -c SRGWJ3YM8W -p | openssl x509 -subject
subject= /UID=A9C7ZD8H7F/CN=Apple Development: <your-email>@gmail.com (TRGW43YM8W)/OU=AS9UV719T7/O=<your name>/C=US
-----BEGIN CERTIFICATE-----
MIIFtTCCBJ2...<rest-of-private-key>
```

Next to `OU` - you will find Team ID. (in our case `AS9UV719T7`).

Now we are good to install our app on a real device. To run/build app for a real device you will need similar to this command to be run in your cargo project:

```sh
crossbundle run apple --release --device --profile-name=aec73e2f-c2f9-4e3b-9393-be19cc52fea3.mobileprovision --team-identifier=AS9UV719T7 --identity=AF96DABFC5DEE81E339ED8755DA8D1E48A87CBFE
```

Now replace test data (`aec73e2f-c2f9-4e3b-9393-be19cc52fea3.mobileprovision`, `AS9UV719T7`, `AF96DABFC5DEE81E339ED8755DA8D1E48A87CBFE`) - with your own and run command. If everything worked well - you will see new app on your device.
